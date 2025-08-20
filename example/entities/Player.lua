return {
  update = function(self, context)
    if context.controller.forward then
      self.y = self.y - 5
    end
    if context.controller.backward then
      self.y = self.y + 5
    end
    if context.controller.left then
      self.x = self.x - 5
    end
    if context.controller.right then
      self.x = self.x + 5
    end
  end
}
