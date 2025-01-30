pub struct DropBomb {
    is_defused: bool,
}

#[allow(dead_code)]
impl DropBomb {
    fn new() -> DropBomb {
        DropBomb { is_defused: false }
    }

    fn defuse(&mut self) {
        self.is_defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.is_defused {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
    }
}
