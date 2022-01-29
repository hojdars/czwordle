# čwordle _[czwordle]_

A Czech port of the popular word game [wordle](https://www.powerlanguage.co.uk/wordle/).

For now, it is a simple command line program, currently looks like this:

![terminal czwordle showcase](https://github.com/hojdars/czwordle/blob/master/czwordle.png)

## GUI

A graphical user interface is in the TODO pipeline.

## Dictionary

The program is not provided with a dictionary. It expects a file called `jmena.txt` in the root directory. The file should have the following structure:

1. one word per one line
2. corpus-like tags are allowed after a forward slash `/`

Example:

```
abeceda/ZQ
pivo/MQR
kozel/PIV
```

The corpus I use is not provided since I have not looked into its license yet. It can be obtained and generated by following the steps in [this blogpost (in Czech language)](http://szj.cz/seznam-ceskych-podstatnych-jmen/).
