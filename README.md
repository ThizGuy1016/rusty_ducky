# rusty_ducky
> "DuckyScript simplified"

## Compatibility

**rusty_ducky** has the ability to transpile DuckyScript into Circuit Python code that can be run on any Circuit Python compatibile microcontroller that has usb hid support.
For a list of compatible devices check this link: [Circuit Python Downloads](https://circuitpython.org/downloads)
> Keep in mind not all devices on this list will work due to usb hid support.

## Installation 

- Head to this site and download circuit python for your microcontroller: [Circuit Python Downloads](https://circuitpython.org/downloads)
- Install circuit python onto yur device following the instructions from here: [Circuit Python Guide](https://learn.adafruit.com/welcome-to-circuitpython/installing-circuitpython)
- Download **rusty_ducky** from the releases for your operating system
- Finished

> wait... you don't have to download the adafruit_hid library... interesting...

## DuckyScript

- Head to this site to check out how to create DuckyScript Payloads: [DuckyScript Site](https://docs.hak5.org/hc/en-us/articles/360010555153-Ducky-Script-the-USB-Rubber-Ducky-language)

## --help

```
OPTIONS:
        
    -i, --template <template>    Overrides rusty ducky's template circuit python file to a file of
                                 your choosing.
    -l, --language <language>    Points rusty ducky to a [keyboard_language].json file to parse. [default: US]
    -o, --output <output>        Specify a name for the transpiled cuircut python file. [default: Code.py]
    
    -p, --payload <payload>      Points rusty ducky to a payload file to transpile. Default is payload.txt [default: payload.txt]
    
    -t, --transpile              Tells rusty ducky to transpile the payload file to cuircut python.
    
    -v, --verbose                Sets the verbosity level of rusty ducky errors.
    
    -V, --version                Print version information
=======
# rusty_ducky
> "DuckyScript simplified"

## Compatibility

**rusty_ducky** has the ability to transpile DuckyScript into Circuit Python code that can be run on any Circuit Python compatibile microcontroller that has usb hid support.
For a list of compatible devices check this link: [Circuit Python Downloads](https://circuitpython.org/downloads)
> Keep in mind not all devices on this list will work due to usb hid support.

## Installation 

- Head to this site and download circuit python for your microcontroller: [Circuit Python Downloads](https://circuitpython.org/downloads)
- Install circuit python onto yur device following the instructions from here: [Circuit Python Guide](https://learn.adafruit.com/welcome-to-circuitpython/installing-circuitpython)
- Download **rusty_ducky** from the releases for your operating system
- Finished

> wait... you don't have to download the adafruit_hid library... interesting...

## DuckyScript

- Head to this site to check out how to create DuckyScript Payloads: [DuckyScript Site](https://docs.hak5.org/hc/en-us/articles/360010555153-Ducky-Script-the-USB-Rubber-Ducky-language)

## --help

```
OPTIONS:
        
    -i, --template <template>    Overrides rusty ducky's template circuit python file to a file of
                                 your choosing.
    -l, --language <language>    Points rusty ducky to a [keyboard_language].json file to parse. [default: US]
    -o, --output <output>        Specify a name for the transpiled cuircut python file. [default: Code.py]
    
    -p, --payload <payload>      Points rusty ducky to a payload file to transpile. Default is payload.txt [default: payload.txt]
    
    -v, --verbose                Sets the verbosity level of rusty ducky errors.
    
    -V, --version                Print version information
