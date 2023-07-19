from collections import Counter
import functools
import numpy as np
from scipy.sparse.csgraph import connected_components
from collections import deque
# import networkx as nx


        # func([], set(NODES), set(), report)

# coding: utf-8

def find_cliques(adj):
    """Returns all maximal cliques in an undirected graph.

    For each node *v*, a *maximal clique for v* is a largest complete
    subgraph containing *v*. The largest maximal clique is sometimes
    called the *maximum clique*.

    This function returns an iterator over cliques, each of which is a
    list of nodes. It is an iterative implementation, so should not
    suffer from recursion depth issues.

    Parameters
    ----------
    G : NetworkX graph
        An undirected graph.

    Returns
    -------
    iterator
        An iterator over maximal cliques, each of which is a list of
        nodes in `G`. The order of cliques is arbitrary.

    See Also
    --------
    find_cliques_recursive
        A recursive version of the same algorithm.

    Notes
    -----
    To obtain a list of all maximal cliques, use
    `list(find_cliques(G))`. However, be aware that in the worst-case,
    the length of this list can be exponential in the number of nodes in
    the graph. This function avoids storing all cliques in memory by
    only keeping current candidate node lists in memory during its search.

    This implementation is based on the algorithm published by Bron and
    Kerbosch (1973) [1]_, as adapted by Tomita, Tanaka and Takahashi
    (2006) [2]_ and discussed in Cazals and Karande (2008) [3]_. It
    essentially unrolls the recursion used in the references to avoid
    issues of recursion stack depth (for a recursive implementation, see
    :func:`find_cliques_recursive`).

    This algorithm ignores self-loops and parallel edges, since cliques
    are not conventionally defined with such edges.

    References
    ----------
    .. [1] Bron, C. and Kerbosch, J.
       "Algorithm 457: finding all cliques of an undirected graph".
       *Communications of the ACM* 16, 9 (Sep. 1973), 575--577.
       <http://portal.acm.org/citation.cfm?doid=362342.362367>

    .. [2] Etsuji Tomita, Akira Tanaka, Haruhisa Takahashi,
       "The worst-case time complexity for generating all maximal
       cliques and computational experiments",
       *Theoretical Computer Science*, Volume 363, Issue 1,
       Computing and Combinatorics,
       10th Annual International Conference on
       Computing and Combinatorics (COCOON 2004), 25 October 2006, Pages 28--42
       <https://doi.org/10.1016/j.tcs.2006.06.015>

    .. [3] F. Cazals, C. Karande,
       "A note on the problem of reporting maximal cliques",
       *Theoretical Computer Science*,
       Volume 407, Issues 1--3, 6 November 2008, Pages 564--568,
       <https://doi.org/10.1016/j.tcs.2008.05.010>

    """
    if len(adj) == 0:
        return

    # adj = {u: {v for v in G[u] if v != u} for u in G}
    Q = [None]

    subg = set(list(adj.keys()))
    cand = set(list(adj.keys()))
    u = max(subg, key=lambda u: len(cand & adj[u]))
    ext_u = cand - adj[u]
    stack = []

    try:
        while True:
            if ext_u:
                q = ext_u.pop()
                cand.remove(q)
                Q[-1] = q
                adj_q = adj[q]
                subg_q = subg & adj_q
                if not subg_q:
                    yield Q[:]
                else:
                    cand_q = cand & adj_q
                    if cand_q:
                        stack.append((subg, cand, ext_u))
                        Q.append(None)
                        subg = subg_q
                        cand = cand_q
                        u = max(subg, key=lambda u: len(cand & adj[u]))
                        ext_u = cand - adj[u]
            else:
                Q.pop()
                subg, cand, ext_u = stack.pop()
    except IndexError:
        pass

class Solution:

    # def pair_intersection(self, pair1, pair2):
    #     x1, y1 = pair1
    #     x2, y2 = pair2
    #     maxp = max(pair1 + pair2)
    #     minp = min(pair1 + pair2)
    #     return maxp - minp < abs(y1-x1) + abs(y2-x2)
        # return maxp - minp < max((x2,y2)) + max((x1,y1))
    def pair_intersection(self, pair1, pair2):
        A, B = pair1
        C, D = pair2
        return abs(A-B-C+D) >= abs(A+B-C-D)

    def longestCommonSubsequenceBase(self, text1: str, text2: str) -> int:
        pairs = []
        for i, x in enumerate(text1):
            for j, y in enumerate(text2):
                if x == y:
                    pairs.append((i,j))
        
        n = len(pairs)
        pair_intersections = {}
        pair_intersectionm = np.zeros((n,n))
        for i, p1 in enumerate(pairs):
            pair_intersections[i] = (set())
            for j, p2 in enumerate(pairs):
                if self.pair_intersection(p1,p2) and i != j:
                    pair_intersections[i].add(j)
                    pair_intersectionm[i,j] = self.pair_intersection(p1,p2)
        
        # pair_intersections = {x:list(pair_intersections[x]) for x in pair_intersections}
        # print(pair_intersections)
        
        # bonkerbosch = bonkerbosch_for_neighbors(pair_intersections)
        # bonkerbosch([], set([x for x in range(len(pair_intersections))]), set())
        cq = sum([1 for _ in find_cliques(pair_intersections)])
        # print(cq)
        # G = nx.from_numpy_matrix(pair_intersectionm)
        # # # all_cliques = ([*nx.enumerate_all_cliques(G)])
        # # n = nx.graph_number_of_cliques(G)
        # cqn = sum([1 for _ in nx.find_cliques(G)])
        # print(cqn)
        return cq
        # # print(all_cliques)
        # # n, labels = connected_components(pair_intersections)
        # # return len(all_cliques)
        # return n
    
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        @functools.lru_cache
        def solve(t1, t2, i, j):
            # if i < 5 and j < 5:
            #     return self.longestCommonSubsequenceBase(t1[:i+1],t2[:j+1])
            if i < 0 or j < 0:
                return 0
            if t1[i] == t2[j]:
                return 1 + solve(t1,t2,i-1,j-1)
            elif i == 0 and j == 0:
                return 1 if t1[:i+1] == t2[:j+1] else 0
                # return 1 if t1[:i+1] in t2 else 0
            # elif j == 0:
                # return 1 if t2[:j+1] in t1 else 0
            else:
                return max(solve(t1,t2,i-1,j),solve(t1,t2,i,j-1))
        return solve(text1,text2,len(text1)-1,len(text2)-1)
    
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        args = (text1,text2,len(text1)-1,len(text2)-1)
        stack = deque()
        ret = np.ones(len(text1,text2)) * -1
        while True:
            (t1,t2,i,j) = args
            # if i < 5 and j < 5:
            #     return self.longestCommonSubsequenceBase(t1[:i+1],t2[:j+1])
            if i < 0 or j < 0:
                deque.appendright((0,(0,)))
            if t1[i] == t2[j]:
                deque.
                return 1 + solve(t1,t2,i-1,j-1)
            elif i == 0 and j == 0:
                return 1 if t1[:i+1] == t2[:j+1] else 0
                # return 1 if t1[:i+1] in t2 else 0
            # elif j == 0:
                # return 1 if t2[:j+1] in t1 else 0
            else:
                return max(solve(t1,t2,i-1,j),solve(t1,t2,i,j-1))
        return solve(text1,text2,len(text1)-1,len(text2)-1)
                

sol = Solution()
# print(sol.longestCommonSubsequence('abcefdg','aecfbdg'))
# print(sol.longestCommonSubsequence('abcde',' ace'))
# # print(sol.longestCommonSubsequence("oxcpqrsvwf","shmtulqrypy"))
print(sol.longestCommonSubsequence("bsbininm","jmjkbkjkv"))
print(sol.longestCommonSubsequence("bsbini","jmjkbkjkv"))
# "oxcpqrsvwf"
# "shmtulqrypy"
