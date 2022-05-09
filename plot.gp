set xlabel "Number of Options"
set ylabel "Number of Nodes"
set logscale y 10
plot "data.dat" using 1:2 title "ITE", \
    "" using 1:3 title "XOR-Ladder"
