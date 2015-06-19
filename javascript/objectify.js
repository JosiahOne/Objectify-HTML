// A javascript-based runtime compiler for objectified HTML code.

var markup;
window.onload = OnLoad();

function OnLoad() {
    markup = document.body.innerHTML;
    console.log(markup);
    RunCompiler();
}

function RunCompiler() {
  var buildContents = GetFileContents(".build");
  var files = GetFilesFromString(buildContents);
  var orig = "";

  while (orig != markup) {
    orig = markup;
    InlineReplaceSelf(files);
  }
}

function InlineReplaceSelf(files) {
  var lookingFor = "<include object=\"";
  var lookingIndex = 0;
  var stopIndex = false;
  var indexPos = 0;
  var startMatching = false;
  var collectedString = "";
  var collecting = true;
   for (var i = 0; i < markup.length; i++) {
     var char = markup[i];

     if (!stopIndex) {
       indexPos = i;
     }

     if (lookingIndex > lookingFor.length && char != ">") {
       if (collecting && char != "\"") {
         collectedString += char;
       } else {
         collecting = false;
       }

       lookingIndex++;
     }
     else if (startMatching && char == lookingFor[lookingIndex]) {
       lookingIndex++;
     }
     else if (char == lookingFor[lookingIndex]) {
       startMatching = true;
       lookingIndex++;
       stopIndex = true;
     } else if (collecting == false && char == ">") {
       // Completed, obtained all relevant data.
       console.log(collectedString);
       markup.slice(indexPos, lookingIndex);
       return;
     }
   }
}

function GetReplacementData(files, objectName) {

}

function GetFileContents(fileName) {
  var xmlhttp;
  if (window.XMLHttpRequest) {// code for IE7+, Firefox, Chrome, Opera, Safari
    xmlhttp = new XMLHttpRequest();
  }
  else {// code for IE6, IE5
    xmlhttp = new ActiveXObject("Microsoft.XMLHTTP");
  }
  xmlhttp.open('GET', fileName, false);  // `false` makes the request synchronous
  xmlhttp.send(null);

  if (xmlhttp.status === 200) {
    return xmlhttp.responseText;
  }
}

function GetFilesFromString(data) {
  var fileNames = GetStringsBetweenStrings("[", "]", data);
  console.log(fileNames);
}

function GetStringsBetweenStrings(stringA, stringB, data) /* -> string[] */ {
  var substrings = [];

  var currentlyMatching = false;
  var tempData = "";

  for(var i = 0; i < data.length; i++) {
    var char = data[i];
    if (currentlyMatching && char != stringB) {
      tempData += char;
    } else if (char == stringB) {
      currentlyMatching = false;
      substrings.push(tempData);
      tempData = "";
    }

    if (char == stringA && !currentlyMatching) {
      currentlyMatching = true;
    }
  }

  return substrings;
}
