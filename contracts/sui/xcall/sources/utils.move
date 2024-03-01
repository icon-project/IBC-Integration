module xcall::utils {
    use std::vector::length;
    use std::vector::borrow;
   
   public fun are_equal<Element>(a1:&vector<Element>,a2:&vector<Element>): bool {

       if(length(a1)!=length(a2)){
            false
       }else{
         let i = 0;
        let len = length(a1);
        while (i < len) {
            if (borrow(a1, i) != borrow(a2,i)) return false;
            i = i + 1;
        };
        true

       }

       

    
}
}