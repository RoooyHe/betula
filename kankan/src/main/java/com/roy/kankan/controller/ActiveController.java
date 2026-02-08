package com.roy.kankan.controller;

import com.roy.kankan.entity.ActiveEntity;
import com.roy.kankan.repository.ActiveRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/api/v1/active")
@RequiredArgsConstructor
public class ActiveController {

  private final ActiveRepository activeRepository;

  @GetMapping
  public ResponseEntity<List<ActiveEntity>> getAllActives() {
    return ResponseEntity.ok(activeRepository.findAll());
  }

  @GetMapping("/{id}")
  public ResponseEntity<ActiveEntity> getActiveById(@PathVariable Long id) {
    return activeRepository
        .findById(id)
        .map(ResponseEntity::ok)
        .orElse(ResponseEntity.notFound().build());
  }

  @PostMapping
  public ResponseEntity<ActiveEntity> createActive(@RequestBody ActiveEntity active) {
    ActiveEntity savedActive = activeRepository.save(active);
    return ResponseEntity.ok(savedActive);
  }

  @PutMapping("/{id}")
  public ResponseEntity<ActiveEntity> updateActive(
      @PathVariable Long id, @RequestBody ActiveEntity activeDetails) {
    return activeRepository
        .findById(id)
        .map(
            active -> {
              active.setTitle(activeDetails.getTitle());
              active.setUserId(activeDetails.getUserId());
              active.setStartTime(activeDetails.getStartTime());
              active.setHidden(activeDetails.getHidden());
              return ResponseEntity.ok(activeRepository.save(active));
            })
        .orElse(ResponseEntity.notFound().build());
  }

  @DeleteMapping("/{id}")
  public ResponseEntity<Void> deleteActive(@PathVariable Long id) {
    if (activeRepository.existsById(id)) {
      activeRepository.deleteById(id);
      return ResponseEntity.ok().build();
    }
    return ResponseEntity.notFound().build();
  }
}
