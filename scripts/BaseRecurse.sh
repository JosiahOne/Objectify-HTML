# BaseRecurse for Unix machines.

mkdir ../OutputDir
cp -r . ../OutputDir/
cd ../OutputDir/
for i in $(find . -name '*.html');
do
    objectify-html -c "$i" >> "$i 2";
    rm "$i";
    mv "$i 2" "$i"
done;
