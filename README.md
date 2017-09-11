# simpleice

Schedule email delivery In Case of Emergency (ICE).

The program expects to find a `.simpleice` configuration file in the home directory of the user, containing details on the email account to use for delivery and the location of the JSON file that contains the mails. **This file can be created using the `create-config` command**.

```
Schedule emails in case of emergency

USAGE:
    simpleice <command>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


ARGS:
    <command>
            Available commands:

            activate            Set delivery date and activate an ICE mail
            create-config       Create empty configuration file
            check               Check if there are scheduled emails to send
            daemon              Run in daemon mode
            deactivate          Deactivate an active ICE mail
            edit                Edit an existing ICE mail
            list                List existing ICE mails
            new                 Create new ICE mail
            remove              Remove an ICE mail
            show                Show details of an ICE mail
```
