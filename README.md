# Redwing

Small cli application to run given command lines at the given path.

# Motivation

Navigating from directory A to directory B to run or check something often leads me to forget my original location. I used to employ the cd <path> && <command> approach, but it consistently modifies my working directory, necessitating the use of cd - to return. It's all quite fatiguing. I'm certain that there must be alternative solutions available, but I felt that I might find a certain level of challenge and enjoyment. This prompted me to create a solution that aligns with my personal preferences. So far, my custom approach has brought me satisfaction in the realm of the command line!

# Install
```bash
$>cargo install --path .
```

## Usage

```bash
$>redwing <path> <command_line>
```

``` bash
$>redwing ../ ls
```

``` bash
$>redwing ~/node_js npm run start &
 ```
