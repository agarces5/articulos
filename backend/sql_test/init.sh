#!/bin/sh
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/create-schema.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/insert-familia.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/insert-articles.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/insert-articles_caja.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/insert-panel.sql &&
/opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/insert-precios.sql &&
# /opt/mssql-tools/bin/sqlcmd -S localhost -U sa -P Sqlserver-2017 < /sql_test/triggers.sql &&
exit
