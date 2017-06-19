#!/usr/bin/gnuplot
reset
set terminal png
set output "clap_perf.png"

set xlabel "Version"
set xrange [0.9:2.1]
set ylabel "Time (ns)"
set yrange [0:35000]

set title "clap-rs Performance (Parse Time - Lower is Better) by Version"
set key inside right top
set grid
set style line 1 lc rgb '#0060ad' lt 1 lw 1 pt 7 ps .5   # --- blue
set style line 2 lc rgb '#dd181f' lt 1 lw 1 pt 5 ps .5   # --- red
set style line 3 lc rgb '#18dd00' lt 1 lw 1 pt 7 ps .5   # --- green
set style line 4 lc rgb '#000000' lt 1 lw 1 pt 5 ps .5   # --- black

plot "clap_perf.dat" u 1:3:4 notitle w yerrorbars ls 1, \
               "" u 1:3 t "Create Parser Using Builder" w lines ls 1, \
               "" u 1:5:6 notitle w yerrorbars ls 2, \
               "" u 1:5 t "Create Parser Usage String" w lines ls 2, \
               "" u 1:7:8 notitle "Parse Complex Args" w yerrorbars ls 3, \
               "" u 1:7 t "Parse Complex Args" w lines ls 3, \
               "" u 1:9:10 notitle w yerrorbars ls 4, \
               "" u 1:9 t "Parse Very Complex Args" w lines ls 4
