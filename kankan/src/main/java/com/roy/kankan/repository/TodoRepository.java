package com.roy.kankan.repository;

import com.roy.kankan.entity.TodoEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;

public interface TodoRepository
    extends JpaRepository<TodoEntity, Long>, JpaSpecificationExecutor<TodoEntity> {}
