// Independent exact solver for selection from a partial order, n <= 10.
// Own code for the referee audit; does not reuse project code.
// Poset: up[x] = bitmask of y with x<y (transitively closed, strict). Ranks 1-based.
#include <bits/stdc++.h>
using namespace std;
typedef unsigned __int128 u128;
typedef unsigned long long u64;

static inline int pc(unsigned x){ return __builtin_popcount(x); }

struct P { int n; uint16_t up[10]; };

static void downs(const P& p, uint16_t* dn){
  for(int x=0;x<p.n;x++) dn[x]=0;
  for(int x=0;x<p.n;x++) for(int y=0;y<p.n;y++) if(p.up[x]>>y&1) dn[y]|=1<<x;
}
// add relation a<b (a,b incomparable) and close transitively
static P addrel(const P& p, int a, int b){
  P q=p; uint16_t dn[10]; downs(p,dn);
  uint16_t below = dn[a] | (1<<a); uint16_t above = p.up[b] | (1<<b);
  for(int u=0;u<p.n;u++) if(below>>u&1) q.up[u] |= above;
  return q;
}

// ---------- canonical form: individualisation-refinement, exhaustive, twins collapsed ----------
struct Canon {
  int n; uint16_t up[10], dn[10];
  u128 best; bool have;
  int refine(int* col, int k){
    while(true){
      int sig[10][21];
      int L = 1+2*k;
      for(int x=0;x<n;x++){
        for(int j=0;j<L;j++) sig[x][j]=0;
        sig[x][0]=col[x];
        for(int y=0;y<n;y++){ if(up[x]>>y&1) sig[x][1+col[y]]++; if(dn[x]>>y&1) sig[x][1+k+col[y]]++; }
      }
      int idx[10]; for(int x=0;x<n;x++) idx[x]=x;
      sort(idx,idx+n,[&](int a,int b){ return lexicographical_compare(sig[a],sig[a]+L,sig[b],sig[b]+L); });
      int nc[10]; int c=0;
      for(int t=0;t<n;t++){
        if(t>0 && lexicographical_compare(sig[idx[t-1]],sig[idx[t-1]]+L,sig[idx[t]],sig[idx[t]]+L)) c++;
        nc[idx[t]]=c;
      }
      int k2=c+1;
      for(int x=0;x<n;x++) col[x]=nc[x];
      if(k2==k) return k;
      k=k2;
    }
  }
  void leaf(const int* col){
    int idx[10]; for(int x=0;x<n;x++) idx[x]=x;
    stable_sort(idx,idx+n,[&](int a,int b){return col[a]<col[b];});
    u128 code=0;
    for(int p=0;p<n;p++) for(int q=0;q<n;q++) if(p!=q){ code=(code<<1)|(u128)((up[idx[p]]>>idx[q])&1); }
    if(!have || code<best){ best=code; have=true; }
  }
  void search(int* col0, int k){
    int col[10]; for(int x=0;x<n;x++) col[x]=col0[x];
    k=refine(col,k);
    for(int c=0;c<k;c++){
      int mem[10], m=0; for(int x=0;x<n;x++) if(col[x]==c) mem[m++]=x;
      if(m<2) continue;
      bool twin=true; for(int t=1;t<m;t++) if(up[mem[t]]!=up[mem[0]]||dn[mem[t]]!=dn[mem[0]]) {twin=false;break;}
      if(twin) continue;
      for(int t=0;t<m;t++){
        int v=mem[t]; bool rep=true;
        for(int s=0;s<t;s++) if(up[mem[s]]==up[v]&&dn[mem[s]]==dn[v]){rep=false;break;}
        if(!rep) continue;
        int nc[10];
        for(int x=0;x<n;x++){ if(col[x]<c) nc[x]=col[x]; else if(col[x]==c) nc[x]=(x==v)?c:c+1; else nc[x]=col[x]+1; }
        search(nc,k+1);
      }
      return;
    }
    leaf(col);
  }
  u128 run(const P& p){
    n=p.n; for(int x=0;x<n;x++) up[x]=p.up[x]; downs(p,dn);
    have=false; int col[10]; for(int x=0;x<n;x++) col[x]=0;
    search(col,1);
    return best;
  }
};

struct H { size_t operator()(const u128& a) const { u64 lo=(u64)a, hi=(u64)(a>>64); return std::hash<u64>()(lo*0x9E3779B97F4A7C15ULL ^ (hi+0x7F4A7C159E3779B9ULL)); } };

struct Table {
  int n;
  vector<P> reps;
  unordered_map<u128,int,H> idx;
  vector<vector<pair<int,int>>> kids;     // per poset: (child x<y, child y<x) per unordered incomparable pair
  vector<vector<pair<int,int>>> kidpairs; // the (x,y) labels on the representative
  vector<array<int8_t,10>> V;
  vector<array<uint32_t,10>> Lmin;
};

static Canon CN;
static int lookup(Table& T, const P& p){
  u128 c=CN.run(p); auto it=T.idx.find(c); if(it==T.idx.end()) return -1; return it->second;
}
static int insert(Table& T, const P& p){
  u128 c=CN.run(p); auto it=T.idx.find(c);
  if(it!=T.idx.end()) return it->second;
  int id=T.reps.size(); T.idx[c]=id; T.reps.push_back(p); return id;
}
static bool solved(const P& p, int i){
  uint16_t dn[10]; downs(p,dn);
  for(int e=0;e<p.n;e++) if(pc(dn[e])==i-1 && pc(p.up[e])==p.n-i) return true;
  return false;
}
static void build(Table& T, int n, bool wantL){
  T.n=n; P e; e.n=n; for(int x=0;x<n;x++) e.up[x]=0;
  insert(T,e);
  for(size_t q=0;q<T.reps.size();q++){
    P p=T.reps[q];
    vector<pair<int,int>> ks, kp;
    for(int x=0;x<n;x++) for(int y=x+1;y<n;y++){
      if((p.up[x]>>y&1)||(p.up[y]>>x&1)) continue;
      int a=insert(T,addrel(p,x,y)); int b=insert(T,addrel(p,y,x));
      ks.push_back({a,b}); kp.push_back({x,y});
    }
    T.kids.push_back(ks); T.kidpairs.push_back(kp);
  }
  int m=T.reps.size();
  vector<int> ncmp(m), ord(m);
  for(int q=0;q<m;q++){ int s=0; for(int x=0;x<n;x++) s+=pc(T.reps[q].up[x]); ncmp[q]=s; ord[q]=q; }
  sort(ord.begin(),ord.end(),[&](int a,int b){return ncmp[a]>ncmp[b];});
  T.V.assign(m,{}); if(wantL) T.Lmin.assign(m,{});
  for(int q: ord){
    const P& p=T.reps[q];
    for(int i=1;i<=n;i++){
      if(solved(p,i)){ T.V[q][i-1]=0; if(wantL) T.Lmin[q][i-1]=1; continue; }
      int bv=1000; uint32_t bl=UINT32_MAX;
      for(auto& kk: T.kids[q]){
        int v=1+max(T.V[kk.first][i-1],T.V[kk.second][i-1]); bv=min(bv,v);
        if(wantL){ uint32_t l=T.Lmin[kk.first][i-1]+T.Lmin[kk.second][i-1]; bl=min(bl,l); }
      }
      T.V[q][i-1]=bv; if(wantL) T.Lmin[q][i-1]=bl;
    }
  }
}

// ---------- weights on a labeled poset ----------
struct W { u64 FG, FGd, C; int64_t Phi; int KEY; };
static W weights(const P& p, int i){
  int n=p.n; uint16_t dn[10]; downs(p,dn);
  W w{0,0,0,0,0};
  unsigned full=(1u<<n)-1;
  for(unsigned S=0;S<=full;S++){
    int k=pc(S); if(k!=i-1 && k!=i) continue;
    bool down=true; for(int x=0;x<n;x++) if((S>>x&1) && (dn[x]&~S)){down=false;break;}
    if(!down) continue;
    if(k==i-1){
      unsigned mins=0; for(int x=0;x<n;x++) if(!(S>>x&1) && !(dn[x]&~S&full)) mins|=1u<<x;
      int b=pc(mins)-1; w.FG+=1ull<<b; w.C+=b+1;
      for(int e=0;e<n;e++) if(mins>>e&1){
        unsigned A=S|(1u<<e); int mx=0; for(int x=0;x<n;x++) if((A>>x&1) && !(p.up[x]&A)) mx++;
        w.KEY=max(w.KEY,(mx-1)+b);
      }
    }
    if(k==i){
      int mx=0; for(int x=0;x<n;x++) if((S>>x&1) && !(p.up[x]&S)) mx++;
      w.FGd+=1ull<<(mx-1);
    }
  }
  w.Phi=(int64_t)w.FG+(int64_t)w.FGd-(int64_t)w.C;
  return w;
}
static int clog2(u64 x){ int d=0; while((1ull<<d)<x) d++; return d; }

#ifndef NO_MAIN
int main(int argc,char**argv){
  int nmax=atoi(argv[1]);          // build tables up to nmax
  int nstats=argc>2?atoi(argv[2]):nmax; // bound checks up to nstats (needs Lmin)
  vector<Table> T(nmax+1);
  for(int n=1;n<=nmax;n++){
    auto t0=chrono::steady_clock::now();
    build(T[n],n,n<=nstats);
    double s=chrono::duration<double>(chrono::steady_clock::now()-t0).count();
    P z; z.n=n; for(int x=0;x<n;x++) z.up[x]=0;
    int e=lookup(T[n],z);
    printf("n=%d posets=%zu time=%.1fs  V_i(E_n):",n,T[n].reps.size(),s);
    for(int i=1;i<=n;i++) printf(" %d",T[n].V[e][i-1]);
    printf("\n"); fflush(stdout);
  }
  // ---- bound checks ----
  for(int n=1;n<=min(nmax,nstats);n++){
    Table& t=T[n]; long long probs=0, viol_phiV=0, viol_keyV=0, viol_phiL=0, viol_CL=0, viol_FGL=0;
    long long eqLeaf=0, unsolved=0, eqLeafRed=0, red=0, viol_local=0, localcases=0, localcases_unsolved=0;
    long long lbTight=0, phiTight=0, keyBeatsPhi=0; double gapLB=0, gapC=0;
    vector<W> cache;
    for(size_t q=0;q<t.reps.size();q++){
      const P& p=t.reps[q]; uint16_t dn[10]; downs(p,dn);
      for(int i=1;i<=n;i++){
        W w=weights(p,i); probs++;
        int V=t.V[q][i-1]; uint32_t Lm=t.Lmin[q][i-1];
        if(clog2(w.Phi)>V) viol_phiV++;
        if(w.KEY>V) viol_keyV++;
        if((u64)w.Phi>Lm) viol_phiL++;
        if(w.C>Lm) viol_CL++;
        if(w.FG>Lm||w.FGd>Lm) viol_FGL++;
        int LB=max(clog2(w.Phi),w.KEY);
        if(LB==V) lbTight++;
        if(clog2(w.Phi)==V) phiTight++;
        if(w.KEY>clog2(w.Phi)) keyBeatsPhi++;
        gapLB+=V-LB; gapC+=V-clog2(w.C);
        bool sv=solved(p,i);
        if(!sv){ unsolved++; if(clog2(w.Phi)==clog2(Lm)) eqLeaf++; }
        bool reduced=true; for(int z=0;z<n;z++) if(pc(p.up[z])>=n-i+1 || pc(dn[z])>=i) reduced=false;
        if(reduced && !sv){ red++; if(clog2(w.Phi)==clog2(Lm)) eqLeafRed++; }
        for(auto& kk: t.kids[q]){
          localcases++; if(!sv) localcases_unsolved++;
          W w1=weights(t.reps[kk.first],i), w2=weights(t.reps[kk.second],i);
          if(w.Phi>w1.Phi+w2.Phi) viol_local++;
        }
      }
    }
    printf("n=%d problems(P up to iso, all i)=%lld unsolved=%lld | viol: ceil(lgPhi)>V %lld, KEY>V %lld, Phi>Lmin %lld, |C|>Lmin %lld, FG|FGd>Lmin %lld\n",
      n,probs,unsolved,viol_phiV,viol_keyV,viol_phiL,viol_CL,viol_FGL);
    printf("     local ineq: (P up to iso, i, labelled unordered incomparable pair) cases=%lld (of which P unsolved for i: %lld) viol=%lld\n",localcases,localcases_unsolved,viol_local);
    printf("     ceil(lgPhi)==ceil(lgLmin): %lld/%lld unsolved (%.4f); reduced&unsolved %lld/%lld (%.4f)\n",eqLeaf,unsolved,(double)eqLeaf/max(1LL,unsolved),eqLeafRed,red,(double)eqLeafRed/max(1LL,red));
    printf("     LB==V %lld/%lld, ceil(lgPhi)==V %lld, KEY>ceil(lgPhi) %lld; avg gap V-LB %.3f, V-ceil(lg|C|) %.3f\n",lbTight,probs,phiTight,keyBeatsPhi,gapLB/probs,gapC/probs);
    fflush(stdout);
  }
  // ---- J inequality: V_i(Q+x) >= 2 + min(V_i(Q),V_{i-1}(Q)), 2<=i<=|Q| ----
  for(int n=1;n+1<=nmax;n++){
    Table& t=T[n]; long long cases=0, viol=0, viol_i1=0;
    for(size_t q=0;q<t.reps.size();q++){
      P p=t.reps[q]; P px=p; px.n=n+1; px.up[n]=0;
      int id=lookup(T[n+1],px);
      for(int i=1;i<=n;i++){
        int lhs=T[n+1].V[id][i-1];
        if(i==1){ if(lhs<2+t.V[q][0]) viol_i1++; continue; }
        cases++; int rhs=2+min(t.V[q][i-1],t.V[q][i-2]);
        if(lhs<rhs){ viol++; if(viol<=3){ printf("  J counterexample |Q|=%d i=%d up=",n,i); for(int x=0;x<n;x++) printf("%x,",p.up[x]); printf(" V_i(Q+x)=%d V_i(Q)=%d V_{i-1}(Q)=%d\n",lhs,t.V[q][i-1],t.V[q][i-2]); } }
      }
    }
    printf("J: |Q|=%d cases(2<=i<=|Q|)=%lld violations=%lld   (rank i=1 fails: %lld)\n",n,cases,viol,viol_i1);
  }
  // ---- reduction invariance ----
  for(int n=2;n<=min(nmax,nstats);n++){
    Table& t=T[n]; long long cases=0, badV=0, badW=0;
    for(size_t q=0;q<t.reps.size();q++){
      const P& p=t.reps[q]; uint16_t dn[10]; downs(p,dn);
      for(int i=1;i<=n;i++){
        W a=weights(p,i);
        for(int z=0;z<n;z++){
          bool small=pc(p.up[z])>=n-i+1, large=pc(dn[z])>=i;
          if(!small&&!large) continue;
          int i2 = small? i-1 : i;
          P r; r.n=n-1; int mp[10]; int c=0; for(int x=0;x<n;x++) if(x!=z) mp[x]=c++;
          for(int x=0;x<n;x++) if(x!=z){ uint16_t u=0; for(int y=0;y<n;y++) if(y!=z && (p.up[x]>>y&1)) u|=1<<mp[y]; r.up[mp[x]]=u; }
          cases++;
          int id=lookup(T[n-1],r);
          if(T[n-1].V[id][i2-1]!=t.V[q][i-1]) badV++;
          W b=weights(r,i2);
          if(a.FG!=b.FG||a.FGd!=b.FGd||a.C!=b.C||a.KEY!=b.KEY) badW++;
        }
      }
    }
    printf("reduction n=%d single-deletion cases=%lld  V changed=%lld  (FG,FGd,|C|,KEY) changed=%lld\n",n,cases,badV,badW);
  }
  return 0;
}
#endif
