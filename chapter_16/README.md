# Chapter 16

I got in a fight with the borrow checker. In the CSG (./shapes/csg.rs) there is a function to filter out intersections `filter_intersections`. I could not get the implementation of `local_intersect` from the `Shape` trait to work if it called the filter function. My solution was to put the logic from the `filter_intersections` in the `local_intersect`.

```text
cannot return value referencing local variable `xs`
returns a value referencing data owned by the current function
```

```rust
impl CSG {
    pub fn filter_intersections<'a>(&'a self, xs: &'a [Intersection]) -> Vec<Intersection> {
        let mut inl = false;
        let mut inr = false;

        let mut results: Vec<Intersection> = Vec::new();

        for i in xs {
            let lhit =
                self.left.id() == i.object.id() || self.left.contains_object_by_id(i.object.id());

            if CSG::intersection_allowed(self.operation, lhit, inl, inr) {
                let c = i.clone();
                results.push(Intersection::new(c.t, c.object));
            }

            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        results
    }
}

impl Shape for CSG {
    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let mut xs:Vec<Intersection> = Vec::new();

        if let Some(left_xs) = self.left.intersect(ray) {
            for i in left_xs {
                xs.push(i);
            }
        }

        if let Some(right_xs) = self.right.intersect(ray) {
            for i in right_xs {
                xs.push(i);
            }
        }

        if xs.len() > 0 {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let f = self.filter_intersections(&xs);
            Some(f)                                              // <---- Error 
        }
        else {
            None
        }
    }
}
```

