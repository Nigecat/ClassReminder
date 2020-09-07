**NOTE:** This program is only useful for a really specific use case so if you have stumbled upon this without being directed here by me then chances are this will be useless for you.

## Usage

Head over to the [releases](https://github.com/Nigecat/ClassReminder/releases) page and go to the latest release. Download the attached `class-reminder.exe` file.  
In the same directory as where you downloaded the file to, create another file called `config.toml`, this is where the application configuration will be stored.  
Copy and paste the following block into your file and change the details:
```
[auth]
username = "firstname.lastname"
password = "your password"

[config]
shortcut = "control+shift+q"
timeout = "5"
```
The username and password fields are for your careylink login details (duh).  
The shortcut is a binding to trigger a notification that displays when the next class is, this is useful for checking how much more time you have until class. The available keys for this are 'control' 'shift' 'alt' and 'super' (windows key).
The timeout is the number of minutes before the class starts you want to notification to appear.

After the configuration is done you can double click the exe to start the program, wait ~5 minutes then hit your shortcut to verify the application loaded the timetable properly/displays the correct class.
If nothing happens please contact me and I can help you debug it.