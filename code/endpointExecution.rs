#[async_recursion]
pub async fn execute(
    &mut self,
    #[cfg(test)] mock_exec_service: &mut ExecutionMockService,
    #[cfg(not(test))] transaction: &mut Transaction<'_, Postgres>,
    endpoint_infos: &Vec<EndpointInfo>,
) -> Result<HashMap<String, Vec<ExecutionResult>>> {
    let mut final_results = HashMap::<String, Vec<ExecutionResult>>::new();

    for query in endpoint_infos {
        #[cfg(not(test))]
        let mut exec = sqlx::query_as::<Postgres, ArbitrarySqlRow>(&query.parsed_sql);
        for var_name in &query.variables {
            let val = self.get_variable_clone(var_name)?;

            #[cfg(not(test))]
            {
                exec = exec.bind(val);
            }
            #[cfg(test)]
            {
                mock_exec_service.bind(&val);
            }
        }

        #[cfg(test)]
        let results = mock_exec_service.simulate_call(&query.parsed_sql);

        #[cfg(not(test))]
        let results = exec
            .fetch_all(&mut *transaction)
            .await?
            .into_iter()
            .map(|it| it.into_map())
            .collect::<Vec<_>>();

        for result in results.into_iter() {
            self.push_execution_map(result);

            #[cfg(test)]
            let children_results = self.execute(mock_exec_service, &query.children).await?;
            #[cfg(not(test))]
            let children_results = self.execute(transaction, &query.children).await?;

            let mut result_map = self
                .pop_execution_map()
                .ok_or(anyhow!("Could not pop execution map"))?;

            // delete private fields
            result_map.retain(|key, _value| key.len() < 8 || &key[0..8] != "private_");

            if final_results.contains_key(&query.name) {
                final_results
                    .get_mut(&query.name)
                    .unwrap()
                    .push(ExecutionResult {
                        data: result_map,
                        children: children_results,
                    })
            } else {
                final_results.insert(
                    query.name.clone(),
                    vec![ExecutionResult {
                        data: result_map,
                        children: children_results,
                    }],
                );
            }
        }
    }

    Ok(final_results)
}
