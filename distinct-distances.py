import argparse
parser = argparse.ArgumentParser(description='Find configurations of points on a 2D grid with distinct distances')

parser.add_argument('grid_size', type=int, help='Grid size')
parser.add_argument('n_points', type=int, help='Number of points')

args = parser.parse_args()

N=args.grid_size
N2=N*N
pts=args.n_points
a=[[[x,y] for y in range(N)] for x in range(N)]
a=sum(a,start=[])

done=False
pp=[i for i in range(pts)]
while(done==False):
    dists=[]
    u=[a[p] for p in pp]
    for q0 in range(len(u)-1):
        for q1 in range(q0+1,len(u)):
            dists.append((u[q0][0]-u[q1][0])**2+(u[q0][1]-u[q1][1])**2)
    dists.sort()
    bad=False
    for j in range(len(dists)-1):
        if (dists[j]==dists[j+1]):
            bad=True
    if (bad==False):
        print(u,dists)

    ix=len(pp)-1
    pp[ix]=1+pp[ix]
    if(pp[ix]==N2):
        pp[ix]=pp[ix]-1
        while(ix>=0 and pp[ix]==N2-len(pp)+ix):
            ix=ix-1
        if (ix==-1):
            done=True
        pp[ix]=1+pp[ix]
        cc=pp[ix]
        for j in range(ix+1,len(pp)):
            cc=cc+1
            pp[j]=cc
