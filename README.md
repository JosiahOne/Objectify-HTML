# Objectify-HTML
[![Build Status](https://travis-ci.org/JosiahOne/Objectify-HTML.svg?branch=parameters)](https://travis-ci.org/JosiahOne/Objectify-HTML)

Objectify-HTML is composed of two primary tools for "compiling" objects/macros in HTML.:

1. An Objective-HTML compiler written in rust that outputs a version of your site with the objects inserted into your pages.
2. A Javascript-based Objective-HTML runtime compiler that "compiles" (meaning, makes the subsitutions) when your page is loaded.

The javascript runtime compiler is recommended when your site will not be used by screenreaders or in other places where JS is not available.

## Why? ##
This project is quite limited, so "why?" is a fair question. This was something I developed because I needed it for work. I was working on a site that wasn't large enough to require me to use PHP-based dynamic webpages, but large enough that I had quite a bit of code duplication. Generally this duplication is accepted by web developers, but since I'm really more of a "software engineer" this really bothered me. This tool is what I use to simplify the implementation of my website. For example:

* I made my footer and header an object, since they were generally identical across the site, so there was no need to duplicate the HTML.
* I use it to break down large HTML pages into several smaller ones. This makes it much easier to work with.
