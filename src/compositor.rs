
trait Drawable {
    fn draw(&self, &Compositor);
}

struct Entity<T> {
    obj: Box<&T>,
}

struct Compositor {
    layers: Vec<Entity,
}
