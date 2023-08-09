Super lightweight CLI for interfacing learning SQL commands. Spins up a SQL server to experiment with.

To build:
`cargo build --release`

To run:
1) Copy "target/release/sqll.exe" to "~/Tools/sqll/" (or similar folder)
2) Make sure "~/Tools/sqll/" is in path
3) Run "sqll help"

Commands may be submitted in-line:
```bash
$ sqll run "SELECT * FROM my_table"
```
Or they may be submitted from a ".sql" file with the `-f` flag:
```bash
$ sqll run -f my_queries.sql
```
No checks are run to keep your computer safe from very big queries, so be careful!