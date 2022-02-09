macro_rules! random_search {
    (
        $best_state: expr,
        $n_iter : expr,
        $target : expr,
        $cost_function: tt,
        $gen_sample: tt,
        
    ) => {{

        let n_iter = $n_iter as usize;
        let reps = $repetition as usize;
        let mut min_cost = $cost_function(&mut $best_state);
        if min_cost <= target { return min_cost; }

        for _ in $n_iter {
            let mut new_state = $gen_sample(&$best_state); 
            let cost = $cost_function(&$best);
            if cost < min_cost {
                $best_state = new_state;
                min_cost = cost;
                if( min_cost <= target ) {
                    return min_cost;
                }
            }
        }

        println!("Search is end, target isn't reached");
        min_cost
    }};
}
