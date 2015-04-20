# BaseRecurse for Unix machines.

mkdir ../OutputDir
cp -r * ../OutputDir/

for i in *;
do
    objectify-html -c &i >> &i;
done;
