(tr --delete '\n' < input.txt) > tmp.txt;
cat tmp.txt > input.txt;
rm tmp.txt;
