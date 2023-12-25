import argparse
parser = argparse.ArgumentParser(description='Find configurations of points on a 2D grid with distinct distances')

parser.add_argument('grid_size', type=int, help='Grid size')
parser.add_argument('n_points', type=int, help='Number of points')

args = parser.parse_args()

import sys
import time

N=args.grid_size
N2=N*N
pts=args.n_points
a=[[(x,y) for y in range(N)] for x in range(N)]
a=sum(a,start=[])

def dist(u,v):
    return sum([(u[i]-v[i])**2 for i in range(len(u))])

def distances(p):
    dd=[]
    for a in range(len(p)-1):
        for b in range(a+1,len(p)):
            dd=dd+[dist(p[a],p[b])]
    return dd

def extend(pl,G):
    if(len(pl)==pts):
        ds=distances(pl)
        ds.sort()
        print("Found ",pl,ds)
        return
#    print("Input ",pl,distances(pl))
    dd0 = {a for a in distances(pl)}
    for vi in range(G,len(a)):
        V=a[vi]
        dd=dd0.copy()
        bad=False
        for w in pl:
            dvw=dist(V,w)
            if dvw in dd:
                bad=True
                break
            dd.add(dvw)
        if (bad==False):
            extend(pl+[V],vi+1)

start = time.time()
for u in range(len(a)-1):
    for v in range(u+1,len(a)):
        print("Start ",a[u],a[v]," t=",time.time()-start,file=sys.stderr)
        extend([a[u],a[v]],v+1)
