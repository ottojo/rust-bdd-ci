set xlabel "Number of Options"
set ylabel "Number of Nodes"
set logscale y 10
plot "data.dat" using 0:1 title "ITE", \
    "" using 0:2 title "XOR-Ladder"
