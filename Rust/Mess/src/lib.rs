pub mod mess {

    use std::collections::HashSet;
    /*
        The Mess trait

        Contains minimal data shared by all basic definitions of a Mess game:

        The State set given by the type T, where C represents a collection to define subsets of T.

        The Option function is of type F : T -> C, which borrows a state and returns the matching subset.

        Meeting other definitions of Mess, such as how to store the starting state and state sequence,
        are left up to the structures implimenting this trait.
    */
    pub trait Mess<T> {
        fn generate_options(&self) -> HashSet<T>;
        fn add_state(&mut self, s:&T);
        fn get_current_state(&self) -> T;
    }

    /*
        Impliment Mess by storing only a starting state and an option function.

        When a new state is generated, replace the starting state with the current state.

        Each state represents the 
    */
    pub struct ProceduralMess<T:Clone, F:Fn(&T) -> HashSet<T>, A:Fn(&T) -> T>
    {
        pub s_n:T,
        c_s:T,
        pub o:F,
        pub a:A
    }

    impl<T, F, A> ProceduralMess<T, F, A> 
    where T : Clone, F : Fn(&T) -> HashSet<T>, A : Fn(&T) -> T
    {
        pub fn new(starting_state:T, option_func:F, applier_func:A) -> ProceduralMess<T, F, A> {
            ProceduralMess { 
                s_n: starting_state.clone(), 
                c_s: starting_state, 
                o: option_func,
                a: applier_func
            }
        }
    }


    impl<T, F, A> Mess<T> for ProceduralMess<T, F, A> 
    where T : Clone, F : Fn(&T) -> HashSet<T>, A : Fn(&T) -> T
    {
        fn generate_options(&self) -> HashSet<T> {
            let o = &self.o;
            let s = &self.s_n;
            o(s)
        }

        fn add_state(&mut self, s:&T) {
            self.s_n = s.to_owned();
            self.c_s = (self.a)(s);
        }

        fn get_current_state(&self) -> T {
            self.c_s.clone()
        }
    }

    pub struct ProceduralRound<T, M:Mess<T>, D:Fn(&HashSet<T>) -> &T> {
        game:M,
        options:HashSet<T>,
        decider:D,
        pub sequence:Vec<T>
    }

    impl<T, M, D> ProceduralRound<T, M, D> 
    where M:Mess<T>, D:Fn(&HashSet<T>) -> &T
    {
        pub fn new(g:M, d:D) -> ProceduralRound<T, M, D>{
            let o = g.generate_options();
            ProceduralRound{
                game: g, 
                options: o, 
                decider: d, 
                sequence: vec![]
            }
        }

        pub fn next(&mut self) -> bool {
            let o:&T = (self.decider)(&self.options);

            self.game.add_state(o);
            self.sequence.push(self.game.get_current_state());

            self.options = self.game.generate_options();

            if self.options.len() == 0 {
                return false;
            }
            true
        } 
    }
}