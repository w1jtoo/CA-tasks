import collections
import math


class Graph:
    def __init__(self, graph: list) -> None:
        self.n = len(graph)
        self.graph = graph
        self.items = []
        self.pair = {}
        self.dist = {}
        self.q = collections.deque()

        self._init_graph()

    def _init_graph(self) -> None:
        for many in self.graph:
            for item in many:
                if item not in self.items:
                    self.items.append(item)

        for v in range(self.n):
            self.pair[v] = None
            self.dist[v] = math.inf

        for v in self.items:
            self.pair[v] = None
            self.dist[v] = math.inf
        

    def hopcroft_karp(self) -> int:
        result = 0
        while self.try_bfs():
            for v in range(self.n):
                if self.pair[v] is None and self.try_dfs(v):
                    result += 1

        return result

    def try_dfs(self, v) -> bool:
        if v is not None:
            for u in self.graph[v]:
                if self.dist[self.pair[u]] == self.dist[v] + 1 \
                        and self.try_dfs(self.pair[u]):
                    self.pair[u] = v
                    self.pair[v] = u
                    return True

            self.dist[v] = math.inf
            return False

        return True

    def try_bfs(self) -> bool:
        for v in range(self.n):
            if self.pair[v] is None:
                self.dist[v] = 0
                self.q.append(v)
            else:
                self.dist[v] = math.inf

        self.dist[None] = math.inf

        while self.q:
            v = self.q.popleft()
            if v is not None:
                for u in self.graph[v]:
                    if self.dist[self.pair[u]] is math.inf:
                        self.dist[self.pair[u]] = self.dist[v] + 1
                        self.q.append(self.pair[u])

        return self.dist[None] is not math.inf


def main():
    sets = read_sets('in.txt')

    graph = Graph(sets)
    total = graph.hopcroft_karp()

    result = []
    if len(sets) != total:
        result.append('N')
    else:
        result.append('Y')
        result.append(' '.join([graph.pair[i] for i in range(len(sets))]))

    write('out.txt', '\n'.join(result))

def write(fname: str, s: str) -> None:
    with open(fname, 'w') as f:
        f.write(s)

def read_sets(fname: str) -> list:
    with open(fname, 'r') as f:
        lines = f.readlines()
        result = []
        for line in lines[1:]:
            result.append(line.strip().split(' ')[:-1])
        return result

    return []

if __name__ == '__main__':
    main()
