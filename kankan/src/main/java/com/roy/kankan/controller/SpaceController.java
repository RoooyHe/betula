package com.roy.kankan.controller;

import com.roy.kankan.entity.SpaceEntity;
import com.roy.kankan.repository.SpaceRepository;
import java.util.List;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/v1/space")
@RequiredArgsConstructor
public class SpaceController {

  private final SpaceRepository spaceRepository;

  @GetMapping
  public ResponseEntity<List<SpaceEntity>> getAllSpaces() {
    return ResponseEntity.ok(spaceRepository.findAll());
  }

  @GetMapping("/{id}")
  public ResponseEntity<SpaceEntity> getSpaceById(@PathVariable Long id) {
    return spaceRepository.findById(id)
            .map(ResponseEntity::ok)
            .orElse(ResponseEntity.notFound().build());
  }

  @GetMapping("/byUserId/{userId}")
  public ResponseEntity<List<SpaceEntity>> findByUserId(@PathVariable String userId) {
    return ResponseEntity.ok(spaceRepository.findByUserId(userId));
  }

  @PostMapping
  public ResponseEntity<SpaceEntity> createSpace(@RequestBody SpaceEntity space) {
    SpaceEntity savedSpace = spaceRepository.save(space);
    return ResponseEntity.ok(savedSpace);
  }

  @PutMapping("/{id}")
  public ResponseEntity<SpaceEntity> updateSpace(@PathVariable Long id, @RequestBody SpaceEntity spaceDetails) {
    return spaceRepository.findById(id)
            .map(space -> {
              space.setTitle(spaceDetails.getTitle());
              space.setUserId(spaceDetails.getUserId());
              space.setCanceled(spaceDetails.getCanceled());
              space.setSort(spaceDetails.getSort());
              space.setColor(spaceDetails.getColor());
              space.setSortBy(spaceDetails.getSortBy());
              return ResponseEntity.ok(spaceRepository.save(space));
            })
            .orElse(ResponseEntity.notFound().build());
  }

  @DeleteMapping("/{id}")
  public ResponseEntity<Void> deleteSpace(@PathVariable Long id) {
    if (spaceRepository.existsById(id)) {
      spaceRepository.deleteById(id);
      return ResponseEntity.ok().build();
    }
    return ResponseEntity.notFound().build();
  }
}
