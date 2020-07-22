pub struct Stack<Element>
where
    Element: Sized,
{
    elements: Vec<Element>,
}

impl<Element> Stack<Element> {
    pub fn push(&mut self, element: Element) {
        self.elements.push(element);
    }
    pub fn pop(&mut self) -> Option<Element> {
        self.elements.pop()
    }
    pub fn peek(&self) -> Option<&Element> {
        self.elements.last()
    }
    pub fn empty(&self) -> bool {
        self.elements.is_empty()
    }
}
