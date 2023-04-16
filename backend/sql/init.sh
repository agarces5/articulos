#!/bin/sh
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/create-schema.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/insert-familia.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/insert-articles.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/insert-articles_caja.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/insert-panel.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/insert-precios.sql && 
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql/triggers.sql && exit
