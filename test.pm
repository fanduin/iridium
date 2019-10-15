load $0 #100
load $1 #1
load $2 #2
test: inc $0
neq $0 $2
jmpe @test
hlt