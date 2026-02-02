package com.roy.kankan.repository;

import com.roy.kankan.entity.ActiveEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;

public interface ActiveRepository
    extends JpaRepository<ActiveEntity, Long>, JpaSpecificationExecutor<ActiveEntity> {}
