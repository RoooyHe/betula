package com.roy.kankan.repository;

import com.roy.kankan.entity.SpaceEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;

import java.util.List;

public interface SpaceRepository
    extends JpaRepository<SpaceEntity, Long>, JpaSpecificationExecutor<SpaceEntity> {
  List<SpaceEntity> findByUserId(String userId);
}
