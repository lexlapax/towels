# Towels

*Towels - jack of all - ala hitchhiker's guide to galaxy towels*

Make programming fun to experiment with, do quick calculations, try out things.. and then move on to your favorite {programming language} dev environment of choice

Start with ruby

- start with a simple dsl driven gui framework like shoes3, then add other simple wrappers
- wrappers include 
    - app - a write and run app to test {language} 
    - box - a write and run (interactive) app specifically for media experimentation
        - audio
        - images
        - video
    - gshell - a full runtime for {language} environment with REPL and output in a gui
        - wshell - similar in a web notebook
    - pluggable wrappers - write your own
    - cmd  - a command line only (the default wrapper)
        - could replace irb or pry but may be heavy weight (besides it's going to rely on them)
        - can be used for replacement of popular shell commands - find, grep, sed etc 
        - also pluggable


Towels is not an IDE

Towels is not shoes3 or shoes3+

Towels is not processing or node box




## Installation

-


## Usage


## Development

##### Cmd Line todo
- file magic
    - identify file type
- Archive
    - Zip/Unzip (zip-rs/zip)
    - bzip/bunzip
    - tar/untar (alexcrichton / tar-rs)
    - gzip (stdlib)

- find files
    - (lff)
- images
    - convert images
    - resize images
- audio
    - convert audio
    - resample audio
- emoji
    - find emoji (veelenga / emoji.cr)
- net
    - http
        - get
        - post
        - download
- calculator
    - bc 
    - currency (money)
    - units (crunits)



-


## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/lexlapax/towels.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).