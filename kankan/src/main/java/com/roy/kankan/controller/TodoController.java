package com.roy.kankan.controller;

import com.roy.kankan.entity.TodoEntity;
import com.roy.kankan.repository.TodoRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/v1/todo")
@RequiredArgsConstructor
public class TodoController {

  private final TodoRepository todoRepository;

  @GetMapping
  public ResponseEntity<List<TodoEntity>> getAllTodos() {
    return ResponseEntity.ok(todoRepository.findAll());
  }

  @GetMapping("/{id}")
  public ResponseEntity<TodoEntity> getTodoById(@PathVariable Long id) {
    return todoRepository
        .findById(id)
        .map(ResponseEntity::ok)
        .orElse(ResponseEntity.notFound().build());
  }

  @PostMapping
  public ResponseEntity<TodoEntity> createTodo(@RequestBody TodoEntity todo) {
    TodoEntity savedTodo = todoRepository.save(todo);
    return ResponseEntity.ok(savedTodo);
  }

  @PutMapping("/{id}")
  public ResponseEntity<TodoEntity> updateTodo(
      @PathVariable Long id, @RequestBody TodoEntity todoDetails) {
    return todoRepository
        .findById(id)
        .map(
            todo -> {
              todo.setTitle(todoDetails.getTitle());
              todo.setCompleted(todoDetails.getCompleted());
              todo.setEndTime(todoDetails.getEndTime());
              todo.setUserId(todoDetails.getUserId());
              todo.setParentId(todoDetails.getParentId());
              return ResponseEntity.ok(todoRepository.save(todo));
            })
        .orElse(ResponseEntity.notFound().build());
  }

  @DeleteMapping("/{id}")
  public ResponseEntity<Void> deleteTodo(@PathVariable Long id) {
    if (todoRepository.existsById(id)) {
      todoRepository.deleteById(id);
      return ResponseEntity.ok().build();
    }
    return ResponseEntity.notFound().build();
  }
}
