
#![allow(warnings)]
use crate::engine::location::Real2D;
use crate::engine::fields::field::Field;


#[derive(Debug)]
enum Axis {
    Vertical,
    Horizontal
}

#[derive(Clone)]
pub struct Kdtree<O: Clone + Copy + PartialEq> {
    pub id: String,
    pub pos_x: f32,
    pub pos_y: f32,
    width: f32,
    height: f32,
    pub locs: Vec<(O, f32, f32)>,
    pub rlocs: Vec<(O, f32, f32)>,
    subtrees: Vec<Self>,
    processors: u32,
    is_leaf: bool,
}

impl<O: Clone + Copy + PartialEq> Kdtree<O> {
    pub fn new(
        id: String,
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        processors: u32,
    ) -> Self {
       Kdtree {
            id,
            pos_x,
            pos_y,
            locs: Vec::new(),
            rlocs: Vec::new(),
            subtrees: Vec::new(),
            width,
            height,
            processors,
            is_leaf: true,
        }
    }

    pub fn create_tree(id:u32, x:f32, y:f32, width: f32, height:f32) -> Self{
        let mut tree = Kdtree::new(id.to_string(), x, y, width, height, 4);
        tree.first_subdivision();
        tree
    }

    pub fn first_subdivision(&mut self){
        const FIRST_SUB_DIMENSION:usize=4;
        let mut count = 0;

        //Root subdivision
        let nodes = self.split(Axis::Vertical, "");
        self.is_leaf=false;
        self.subtrees.push(nodes.0);
        self.subtrees.push(nodes.1);

        for i in 0..FIRST_SUB_DIMENSION/2{
            let mut id = self.id.clone();
            id.push_str("_".to_string().as_str());
            id.push_str((i+3).to_string().as_str());
            let x = self.subtrees[i].split(Axis::Horizontal, id.as_str());
            self.subtrees[i]=x.0;
            self.subtrees.push(x.1);

        }
     
        count+=FIRST_SUB_DIMENSION as u32;

        //Progressive subdivision
        while count<self.processors{
            let mut leaves = self.get_leaves_mut();
            for node in leaves.iter_mut(){
                if count >= node.processors{break;}
                let nodes=node.split(Axis::Vertical, "");
                node.is_leaf=false;
                node.subtrees.push(nodes.0);
                node.subtrees.push(nodes.1);
                count+=1;
                for i in 0..FIRST_SUB_DIMENSION/2{
                    if count >= node.processors{break;}
                    let mut id = node.id.clone();
                    id.push_str("_".to_string().as_str());
                    id.push_str((i+3).to_string().as_str());
                    let x = node.subtrees[i].split(Axis::Horizontal, id.as_str());
                    node.subtrees[i]=x.0;
                    node.subtrees.push(x.1);
                    count+=1
                }
            }
        }
    }

    fn split(&mut self, direction:Axis, id_node:&str) -> (Kdtree<O>, Kdtree<O>){

        let mut id = self.id.clone();
        let mut node_x = self.pos_x;
        let mut node_y = self.pos_y;
        let mut node_w = self.width;
        let mut node_h = self.height;


        let mut n1 = self.clone();
        n1.locs.clear();

        match direction {
            Axis::Vertical => {
                n1.id.push_str("_1".to_string().as_str());
                id.push_str("_2".to_string().as_str());
                n1.width=n1.width/2.0;
                node_x = self.pos_x + self.width/2.0;
                node_w = self.width/2.0;
            },
            Axis::Horizontal => {
                id=id_node.to_string();
                self.height=self.height/2.0;
                n1.height=self.height;
                node_y = self.pos_y + self.height;
                node_h = self.height;
            },
        }
        let agents: Vec<(O,f32,f32)>=Vec::new();
        let p = self.processors;
        let mut n2 = Kdtree::new(id, node_x, node_y, node_w, node_h, p);
        self.is_leaf=false;

        return (n1,n2);
    }

    fn split_on_median(&mut self, median:f32, direction:bool, id_node:&str) -> (Kdtree<O>, Kdtree<O>){

        let mut id = self.id.clone();
        let mut node_x = self.pos_x;
        let mut node_y = self.pos_y;
        let mut node_w = self.width;
        let mut node_h = self.height;
        let mut dir = Axis::Vertical;

        if direction{ dir = Axis::Horizontal}
        else {dir = Axis::Vertical}


        //println!("Axis è {:?}",dir);
        let mut n1 = self.clone();
        self.is_leaf=false;
        n1.locs.clear();

        match dir{
            Axis::Vertical => {
                n1.id.push_str("_1".to_string().as_str());
                id.push_str("_2".to_string().as_str());
                n1.width=median-self.pos_x;
                node_x = median;
                println!("self.pos_x {}, median vale {}, self.width {}, n1.width {}",self.pos_x,  median, self.width,n1.width);
                node_w=self.width-n1.width;
            },

            Axis::Horizontal => {
                id=id_node.to_string();
                n1.height=median-self.pos_y;
                node_y = median;
                println!("self.pos_y {}, median vale {}, self.height {}, n1.height {}",self.pos_y,  median, self.height,n1.height);
                node_h = self.height-n1.height;
            },
        }
        let agents: Vec<(O,f32,f32)>=Vec::new();
        let p = self.processors;
        let mut n2 = Kdtree::new(id, node_x, node_y, node_w, node_h, p);

        println!("La divisione ha generato l'albero {} in {};{} con w: {} e h:{} e is_leaf: {}", n1.id, n1.pos_x,n1.pos_y,n1.width,n1.height, n1.is_leaf);
        println!("La divisione ha generato l'albero {} in {};{} con w: {} e h:{} e is_leaf: {}", n2.id, n2.pos_x,n2.pos_y,n2.width,n2.height, n2.is_leaf);
        return (n1,n2);
    }

    fn get_leaves_mut(&mut self) -> Vec<&mut Kdtree<O>>{
        let mut leaves:Vec<&mut Kdtree<O>> = Vec::new();
        
        for tree in self.subtrees.iter_mut(){
            if tree.is_leaf{
                leaves.push(tree);
            }
            else{
                leaves.append(&mut tree.get_leaves_mut());
            }
        }
        leaves
    }

    pub fn insert(&mut self, agent: O, x: f32, y: f32) {

        if self.is_leaf{
            if self.contains(x, y){
                //println!("inserisco agente {};{} in {}", x,y,self.id);
                /* TO DO rimpiazzare un agente se è già presente */
                self.locs.push((agent,x,y));
            }
        }
        else{
            for tree in self.subtrees.iter_mut(){
                tree.insert(agent, x, y);
            }
        }

        /*let len = self.agents.len();
        if !self.contains(x, y) {
            //println!("ritorno falso");
            return false;
        }
        if self.contains(x, y){
            self.agents.push((agent,x,y));
            println!("Inserito agente in posizione {};{} del nodo {};{}", x, y, self.pos_x, self.pos_y);
            return true;
        }*/
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        //println!("Confronto {};{} con {};{}", self.pos_x, self.pos_y, x, y);
        self.pos_x <= x
            && x < self.pos_x + self.width
            && self.pos_y <= y
            && y < self.pos_y + self.height
    }

    pub fn query_by_location(&mut self, x: f32, y:f32) -> Option<O>{
        let mut option:Option<O> = None;
        if !self.is_leaf{
            for p in self.subtrees.iter_mut(){
                option=p.query_by_location(x, y);
                match option{
                    Some(loc) => {return option;}
                    None =>{continue;}
                }
            }
        }
        else if self.contains(x, y){
            for r in self.rlocs.iter(){
                if r.1==x && r.2==y{
                    option=Some(r.0);
                    return option;
                }
            }
        }
        option
    }

    pub fn get(&self, object:&O, x:f32, y:f32) -> Option<O>{
        if self.contains(x,y){
            for agent in self.locs.iter(){
                if agent.1 == x && agent.2 == y{
                    Some(agent.clone());
                }
            }
        }
        else{
            for subtree in self.subtrees.iter(){
                subtree.get(object, x, y);
            }
        }
        None
    }

    pub fn get_location(&self, object:O) -> Option<Real2D>{
        for agent in self.locs.iter(){
            if agent.0 == object{
                Some(Real2D{x: agent.1, y: agent.2});
            }
        }
        for subtree in self.subtrees.iter(){
            subtree.get_location(object);
        }
        None
    }

    pub fn query_by_location_unbuffered(&mut self, x: f32, y:f32) -> Option<O>{
        let mut option:Option<O> = None;
        if !self.is_leaf{
            for p in self.subtrees.iter_mut(){
                option=p.query_by_location_unbuffered(x, y);
                match option{
                    Some(loc) => {return option;}
                    None =>{continue;}
                }
            }
        }
        else if self.contains(x, y){
            for r in self.locs.iter(){
                if r.1==x && r.2==y{
                    option=Some(r.0);
                    return option;
                }
            }
        }
        option
    }

    pub fn remove_from_location(&mut self, x:f32, y:f32){
        if !self.is_leaf{
            for p in self.subtrees.iter_mut(){
                p.remove_from_location(x, y);
            }
        }
        else if self.contains(x,y){
            for r in self.locs.clone().iter(){
                if r.1==x && r.2==y{
                    self.locs.retain(|&o| o.1!=x && o.2!=y);
                    println!("Rimosso {};{}", x,y);                    
                }
            }
        }
    }

    /*pub fn get_all_agents(&mut self) -> Vec<(O, f32,f32)>{
        let mut all_agents: Vec<(O,f32,f32)> = Vec::new();

        if !self.is_leaf {
            for node in self.subtrees.iter_mut() {
                all_agents.append(&mut node.get_all_agents());
            }
        } else {
            all_agents.append(&mut self.locs.clone());
        }
        all_agents.sort_by(|a,b| a.1.cmp(&b.1));
        all_agents
    }*/
    

    fn calculate_median(&self, agents: &Vec<(O,i32,i32)>) -> i32{
        let len = agents.len();
        
        if len >=1 && len%2==1{
            return agents[len/2].1;
        }

        else if len >=1{
            return (agents[len/2-1].1 + agents[(len/2)].1) / 2
        }
        else{
            return 0;
        }
    }

    fn calculate_median_on_y(&self, agents: &Vec<(O,i32,i32)>) -> i32{
        let len = agents.len();
        
        if len >1 && len%2==1{
            return agents[len/2].2;
        }

        else if len>1{
            return (agents[len/2-1].2 + agents[(len/2)].2) / 2
        }
        else{
            return 0;
        }
    }

    /*fn reshape_tree(&mut self, medians:Vec<i32>){
        const FIRST_SUB_DIMENSION:usize=4;
        let mut count = 0;
        let agents = self.get_all_agents();
        self.width=200;
        self.height=200;
        self.subtrees.clear();
        self.is_leaf=true;

        //Root subdivision
        let nodes = self.split_on_median(medians[0], false, "");
        self.is_leaf=false;
        self.subtrees.push(nodes.0);
        self.subtrees.push(nodes.1);

        for i in 0..FIRST_SUB_DIMENSION/2{
            let mut id = self.id.clone();
            id.push_str("_".to_string().as_str());
            id.push_str((i+3).to_string().as_str());
            let x = self.subtrees[i].split_on_median(medians[i+1], true, id.as_str());
            self.subtrees[i]=x.0;
            self.subtrees.push(x.1);

        }
        
        let mut direction=false;
        let processors = self.processors.clone();
        
        count+=4;
        
        while count < self.processors{
            let mut leaves = self.get_leaves_mut();

            if direction{direction=false;}
            else {direction=true;}

            //println!("Direction è {} e leaves è {}", direction, leaves.len());
            let mut index:Vec<i32>= vec![0,1,-1,0];
            let len = leaves.len();
            for i in 0..len/4{
                let mut ix=index.clone();
                index.append(&mut ix);
            }
            for i in index.iter(){
                println!("In index c'è {}",i);
            }
            let mut ind = 0;
            for node in leaves.iter_mut(){
                println!("Calcolo {}",node.id);
                let subnodes = node.split_on_median(medians[(count-1) as usize], false, count.to_string().as_str());
                count+=1;
                ind+=1;
                node.is_leaf=false;
                node.subtrees.push(subnodes.0);
                node.subtrees.push(subnodes.1);
                println!("Aggiunti {} e {}",node.subtrees[0].id, node.subtrees[1].id);
                if count >= processors {break;}
            }
            if count >= processors {break;}
            for node in leaves.iter_mut(){
                println!("Calcolo {} nel secondo for",node.id);
                if count >= processors {break;}       
                for i in 0..FIRST_SUB_DIMENSION/2{
                    println!("Calcolo {} nel terzo for",node.subtrees[i].id);
                    let mut id = node.id.clone();
                    id.push_str("_".to_string().as_str());
                    id.push_str((i+3).to_string().as_str());
                    let subnodes = node.subtrees[i].split_on_median(medians[(count-1) as usize], true, id.as_str());
                    count+=1;
                    node.subtrees[i]=subnodes.0;
                    node.subtrees.push(subnodes.1);
                    println!("Modificato {} nel terzo for e aggiunto {}",node.subtrees[i].id, node.subtrees.last().unwrap().id);
                    if count >= processors {break;}
                }
            }
        }

        
        //Progressive subdivision
        /*while count<self.processors{
            let mut leaves = self.get_leaves_mut();
            for node in leaves.iter_mut(){
                if count >= node.processors{break;}
                let nodes=node.split_on_median(medians[(count-1) as usize], Axis::Vertical, "_1");
                node.is_leaf=false;
                node.subtrees.push(nodes.0);
                node.subtrees.push(nodes.1);
                count+=1;
                for i in 0..FIRST_SUB_DIMENSION/2{
                    if count >= node.processors{break;}
                    let mut id = node.id.clone();
                    id.push_str("_".to_string().as_str());
                    id.push_str((i+3).to_string().as_str());
                    let x = node.subtrees[i].split_on_median(medians[(count-1) as usize], Axis::Horizontal, id.as_str());
                    node.subtrees[i]=x.0;
                    node.subtrees.push(x.1);
                    count+=1
                }
            }
        }
        println!("La length di agents è {}",agents.len());
        for agent in agents{
            self.insert(agent.0, agent.1, agent.2);
        }*/
        for agent in agents{
            self.insert(agent.0, agent.1, agent.2);
        }
        self.print_leaves();
    }*/

    pub fn apply_to_all_values<F>(&mut self, closure: &F)
    where
        F: Fn(&(O, f32, f32)) -> (O, f32,f32),
    {
        if !self.is_leaf{
            for p in self.subtrees.iter_mut(){
                p.apply_to_all_values(closure);
            }
        }
        else if !self.rlocs.is_empty(){
            for elem in self.rlocs.iter() {
                let result = closure(elem);
                self.locs.retain(|&x| x.1!=result.1);
                self.locs.push(result);
            }
        }
    }

    pub fn get_neighbors_within_distance(&self, loc:Real2D, distance:f32) -> Vec<(O,f32,f32)>{
        let mut neighbors:Vec<(O, f32, f32)> = Vec::new(); 

        if self.is_leaf{
            for agent in &self.rlocs{
                if f32::abs(agent.1 - loc.x)<distance && f32::abs(agent.2 - loc.y)<distance{
                    neighbors.push(agent.clone());
                }
            }
        }
        else{
            for tree in &self.subtrees{
                neighbors.append(&mut tree.get_neighbors_within_distance(loc, distance));
            }
        }

        neighbors
    }

    

    fn balance(&self, agents: &mut Vec<(O,i32,i32)>, direction:bool) -> (Vec<Vec<(O,i32,i32)>>, Vec<i32>){
        let len = agents.len();
        let mut medians: Vec<i32> = Vec::new();
        /*for i in 0..len{
            println!("Mi preparo a bilanciare {};{}", agents[i].1, agents[i].2)
        }*/
        let mut vec_right = agents.split_off(len/2);
        let mut median_right=0;
        let mut median_left=0;

        if direction{
            vec_right.sort_by(|a,b| a.1.cmp(&b.1));
            agents.sort_by(|a,b| a.1.cmp(&b.1));
            median_right = self.calculate_median(&vec_right);
            median_left = self.calculate_median(agents);
            println!("mediana x sinistra {} ",median_left);
            println!("mediana x destra {} ",median_right);
        }
        else{
            vec_right.sort_by(|a,b| a.2.cmp(&b.2));
            agents.sort_by(|a,b| a.2.cmp(&b.2));
            median_right = self.calculate_median_on_y(&vec_right);
            median_left = self.calculate_median_on_y(agents);
            println!("mediana y sinistra {} ",median_left);
            println!("mediana y destra {} ",median_right);
        }
        medians.push(median_left);
        medians.push(median_right);
        let mut vec: Vec<Vec<(O,i32,i32)>> = Vec::new();
        vec.push(agents.to_vec());
        vec.push(vec_right);
        return (vec,medians);
    }

    /*pub fn balance_tree_def(&mut self){
        let mut all_agents=self.get_all_agents();
        let mut all_medians: Vec<i32> = Vec::new();
        let mut count=all_agents.len();
        let mut all_x=0;

        let median = self.calculate_median(&all_agents);
        all_medians.push(median);
        println!("Dovrei dividere in posizione {} al centro", median);

        let mut vec: Vec<Vec<(O,i32,i32)>> = Vec::new();
        vec.push(all_agents);
        let mut direction = true;
        let mut iterations = 0;

        //true = x
        //false = y

        while iterations<self.processors{
            let len = vec.len();
            let mut vec_temp:Vec<Vec<(O,i32,i32)>> = Vec::new();
            
            if direction{direction=false}
            else {direction=true}

            for i in (0..len){
                iterations+=2;
                let balance_results = &mut self.balance(&mut vec[i], direction);
                vec_temp.append(&mut balance_results.0);
                all_medians.append(&mut balance_results.1);
                if iterations> self.processors{break;}
            }
            /*for i in (1..len).step_by(2){
                iterations+=2;
                let balance_results = &mut self.balance(&mut vec[i], direction);
                vec_temp.append(&mut balance_results.0);
                all_medians.append(&mut balance_results.1);
                if iterations> self.processors{break;}
            }*/
            vec=vec_temp.clone();
            vec_temp.clear();
        }
        //reshape the tree with the new values
        self.reshape_tree(all_medians);
    }
*/
    pub fn tree_state(&self){
        if !self.is_leaf{
            for node in self.subtrees.iter(){
                println!("Il nodo {} ha {} agenti all'interno", node.id, node.locs.len())
            }
        }
    }

    pub fn print_leaves(&self){
        if !self.is_leaf{
            for node in self.subtrees.iter(){
                node.print_leaves();
            }
        }
        else{
            println!("Nodo: {}, in {};{} e w: {}, h: {} e ha {} agenti all'interno di locs e {} in rlocs", self.id, self.pos_x, self.pos_y, self.width, self.height, self.locs.len(), self.rlocs.len());
        }
    }
}

impl<O: Eq + Clone + Copy> Field for Kdtree<O> {
    fn lazy_update(&mut self){
        unsafe {
            std::ptr::swap(
                &mut self.locs,
                &mut self.rlocs,
            )
        }
        self.locs.clear();
        if !self.is_leaf {
            for i in self.subtrees.iter_mut(){
                i.lazy_update();
            }
        }      
    }

    fn update(&mut self){
        let mut rlocs_clone=self.rlocs.clone();
        self.locs.append(&mut rlocs_clone);
        if !self.is_leaf {
            for i in self.subtrees.iter_mut(){
                i.lazy_update();
            }
        }      
    }
}