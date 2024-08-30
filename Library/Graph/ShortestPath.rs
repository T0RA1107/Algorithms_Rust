fn dijkstra(N: usize, G: &Vec<Vec<usize>>, st: usize) -> Vec<usize> {
    let INF = 1 << 60;
    let mut dist = vec![INF; N + 1];
    dist[st] = 0;
    let mut hq = std::collections::BinaryHeap::new();
    hq.push((std::cmp::Reverse(0), st));
    while let Some((std::cmp::Reverse(d), p)) = hq.pop() {
        if d > dist[p] { continue; }
        for &q in &G[p] {
            if dist[q] > d + 1 {
                dist[q] = d + 1;
                hq.push((std::cmp::Reverse(dist[q]), q));
            }
        }
    }
    return dist;
}

fn warshall_floyd(N: usize, G: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<usize>> {
    let INF = 1 << 60;
    let mut dist = vec![vec![INF; N]; N];
    for u in 0..N {
        dist[u] = 0;
        for &(v, w)in &G[u] {
            dist[u][v] = w;
        }
    }
    for k in 0..N {
        for j in 0..N {
            for i in 0..N {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    dist
}
