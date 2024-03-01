Bill Tracker is a CLI application for tracking monthly bills that are due.
With input file, bill tracker is an easy tool to keep track of each monthly bill
with information on paid status, due date, and amount. When running the app, the user
can update bill information by adding/deleting bills, editing specific bills, marking
bills as paid or unpaid, and viewing amounts paid and due. 

bills.txt is the defualt file to read in bill information and update when complete.
It is a csv file with these values: [bill name], [bill amount], [due date], [paid status].
After running the app and making changes to bill infomration, bills.txt is re-written
to, saving the latest bill information.

This CLI app is written 100% in Rust using Cargo for building and deploying.
