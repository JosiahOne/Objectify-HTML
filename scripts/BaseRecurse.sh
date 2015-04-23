# BaseRecurse for Unix machines.

mkdir ../OutputDir
cp -r . ../OutputDir/

for i in ../OutputDir/*.html;
do
    objectify-html -c "$i" >> "$i 2";
    rm "$i";
    mv "$i 2" "$i"
done;
