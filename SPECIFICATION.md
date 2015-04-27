# The "Objectified-HTML" specification
This document outlines how the compiler assumes your objectified documents are laid out and how the minimal language is implemented.

## 1.0 - Terminology

## 2.0 - General Specification
In this section we will provide the general specification of the compiler and how it expects documents to be laid out. This section should not be confused with 3.0, aka, the Technical Specification, which goes into extreme detail about each component. That includes, O(n) running time of each algorithm, error handling, and limited capabilities.

### 2.1 - .build files
Build files list the specific .ohtml files you want to parse to locate objects. .ohtml files are not searched unless they exist in this file.

By default, objectify-html searches the directory you run it in for |.build|. BaseRecurse.sh only uses the .build. If you want to be more specific, you can specify the option with a |-b| flag.

When the search happens, the files are looked at in order of how they appear in the .build file. Once the object is found, we stop searching. The locations are found by getting any substrings between '[' and ']'.
File example:
---------------
[file.ohtml]
[dir/a.ohtml]
[dir/b.ohtml]
---------------
### 2.2 - HTML files

Objects are inserted into HTML files using an |<include>| tag. The tag takes ONE parameter, and must be inserted with the inline closing format. |<foo bar="fizz"/>| not |<foo bar="fizz"></foo>|!

You specify the specific object using the |object| attribute. Obviously, there must be a space between the "<include" and the "object=". So for example, a proper include would look like |<include object="foo"/>|.

### 2.3 - OHTML files

OHTML files, or Object-based Hyper Text Markup Language, actually defines the subsitutions. You create .ohtml files and reference them in your .build file. Objects are created by inserting the HTML in an <begin> tag along with the object attribute. In this case, you do NOT use inline closing tags, but instead </begin> at the end of the definition. For example:
-------------------------------------
<begin object="foo">
    <div class="thing"> Test </div>
</begin>
-------------------------------------

You can declare multiple objects in a file and you can insert <include> tags in the definitions.

### 2.4 - Command Syntax

