const editors: EditorTree = {
  "Data management": {
    icon: faDatabase,
    editors: {
      "Table data management": { icon: faTable, component: TableDataManagement, },
      "Data SQL editor": { icon: faCode, component: DataSqlEditor },
    },
  },
  "Schema editing": {
    icon: faTable,
    editors: {
      "Table creation": { icon: faPlus, component: TableCreation },
    },
  },
  "Endpoint management": {
    icon: faCode,
    editors: {
      "Endpoint management": { icon: faCogs, component: EndpointManagement },
    },
  },
  "User management": {
    icon: faUsers,
    editors: {
      "User creation": { icon: faUserPlus, component: UserCreation },
      "User management": { icon: faUsersCog, component: UserManagement },
    },
  },
};
