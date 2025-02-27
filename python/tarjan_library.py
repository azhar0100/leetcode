
class TarjanGraphNode:
    def __init__(self,value):
        self.index = None
        self.lowlink = None
        self.in_stack = None
        self.value = value
    
    def __str__(self) -> str:
        index = self.index
        lowlink = self.lowlink
        in_stack = self.in_stack
        value = self.value
        return f"TGN({value},i={index},l={lowlink},s={in_stack})"

    def __repr__(self) -> str:
        return str(self)
    

def stronglyconnect(node, adj, index=0, stack=[], strongly_connected_components = []):
    v = node
    v.index = index
    v.lowlink = index

    current_index = index + 1

    stack.append(v)
    v.in_stack = True

    for w in adj[node]:
        if w.index is None:
            current_index, strongly_connected_components = stronglyconnect(w, adj, index=current_index, stack=stack)
            v.lowlink = min(w.lowlink, v.lowlink)
        elif w.in_stack:
            v.lowlink = min(v.lowlink, w.index)
    
    if v.lowlink == v.index:
        strongly_connected_component = []
        while True:
            w = stack.pop()
            w.in_stack = False
            strongly_connected_component.append(w)
            if w == v:
                break
        strongly_connected_components.append(strongly_connected_component)
    return current_index, strongly_connected_components

def tarjan(adjacency_dict):
    nodes = {x:TarjanGraphNode(x) for x in adjacency_dict.keys()}
    adj = {nodes[k]:[nodes[x] for x in v] for k,v in adjacency_dict.items()}
    index = 0
    for node in nodes.values():
        if node.index is None:
            final_index, strongly_connected_components = stronglyconnect(node, adj, index)
            index = final_index + 1
    final_result = []
    for component in strongly_connected_components:
        final_result.append([x.value for x in component])
    return final_result


result = tarjan({
    0 : [1,2],
    1: [0,2,3],
    2: [0,1],
    3: [1,4,5],
    4: [3,5],
    5: [3,4]
})
print(result)


