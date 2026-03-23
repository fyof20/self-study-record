<template>
  <div class="tasks">
    <div class="container">
      <h2>任务规划</h2>
      
      <!-- 任务操作栏 -->
      <div class="task-actions">
        <button class="btn btn-primary" @click="showAddTaskDialog = true">添加任务</button>
        <button class="btn btn-success">智能排期</button>
        <button class="btn btn-warning">批量操作</button>
      </div>
      
      <!-- 任务筛选 -->
      <div class="task-filters">
        <select class="form-control" v-model="filterStatus">
          <option value="all">全部任务</option>
          <option value="pending">未开始</option>
          <option value="inProgress">进行中</option>
          <option value="completed">已完成</option>
          <option value="overdue">逾期</option>
        </select>
        <select class="form-control" v-model="filterPriority">
          <option value="all">全部优先级</option>
          <option value="high">高</option>
          <option value="medium">中</option>
          <option value="low">低</option>
        </select>
      </div>
      
      <!-- 任务列表 -->
      <div class="card">
        <h3>我的任务</h3>
        <div class="task-list">
          <div v-for="task in filteredTasks" :key="task.id" class="task-card" :class="task.status">
            <div class="task-header">
              <h4>{{ task.title }}</h4>
              <div class="task-priority" :class="task.priority">
                {{ getPriorityText(task.priority) }}
              </div>
            </div>
            <div class="task-body">
              <div class="task-info">
                <span class="info-item">📅 截止时间: {{ task.deadline }}</span>
                <span class="info-item">⏰ 预估耗时: {{ task.estimatedTime }}小时</span>
                <span class="info-item">📚 关联课程: {{ task.course || '无' }}</span>
              </div>
              <div class="task-description">{{ task.description }}</div>
              <div class="task-progress" v-if="task.status === 'inProgress'">
                <div class="progress-bar">
                  <div class="progress-fill" :style="{ width: task.progress + '%' }"></div>
                </div>
                <span class="progress-text">{{ task.progress }}%</span>
              </div>
            </div>
            <div class="task-footer">
              <div class="task-status">
                <span class="status-tag" :class="task.status">
                  {{ getStatusText(task.status) }}
                </span>
              </div>
              <div class="task-actions">
                <button class="btn btn-sm btn-primary" @click="editTask(task)">编辑</button>
                <button class="btn btn-sm btn-success" @click="toggleTaskStatus(task)">
                  {{ task.status === 'completed' ? '标记未完成' : '标记完成' }}
                </button>
                <button class="btn btn-sm btn-danger" @click="deleteTask(task.id)">删除</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 添加任务对话框 -->
      <div v-if="showAddTaskDialog" class="dialog-overlay" @click="showAddTaskDialog = false">
        <div class="dialog-content" @click.stop>
          <h3>添加任务</h3>
          <div class="form-group">
            <label class="form-label">任务标题</label>
            <input type="text" class="form-control" v-model="newTask.title">
          </div>
          <div class="form-group">
            <label class="form-label">任务描述</label>
            <textarea class="form-control" v-model="newTask.description" rows="3"></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">截止时间</label>
            <input type="date" class="form-control" v-model="newTask.deadline">
          </div>
          <div class="form-group">
            <label class="form-label">预估耗时（小时）</label>
            <input type="number" class="form-control" v-model="newTask.estimatedTime">
          </div>
          <div class="form-group">
            <label class="form-label">优先级</label>
            <select class="form-control" v-model="newTask.priority">
              <option value="high">高</option>
              <option value="medium">中</option>
              <option value="low">低</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">关联课程</label>
            <input type="text" class="form-control" v-model="newTask.course">
          </div>
          <div class="dialog-actions">
            <button class="btn btn-primary" @click="addTask">确定</button>
            <button class="btn" @click="showAddTaskDialog = false">取消</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

// 模拟数据
const tasks = ref([
  { id: 1, title: '完成数学作业', description: '第三章习题1-10题', deadline: '2026-03-25', estimatedTime: 2, priority: 'high', status: 'pending', course: '高等数学' },
  { id: 2, title: '准备英语演讲', description: '关于环保主题的5分钟演讲', deadline: '2026-03-24', estimatedTime: 3, priority: 'medium', status: 'inProgress', progress: 50, course: '大学英语' },
  { id: 3, title: '编程作业', description: '完成Java实验报告', deadline: '2026-03-26', estimatedTime: 4, priority: 'high', status: 'pending', course: '计算机基础' },
  { id: 4, title: '复习高等数学', description: '第三章知识点回顾', deadline: '2026-03-27', estimatedTime: 2, priority: 'medium', status: 'completed', course: '高等数学' }
])

const showAddTaskDialog = ref(false)
const newTask = ref({
  title: '',
  description: '',
  deadline: '',
  estimatedTime: 1,
  priority: 'medium',
  status: 'pending',
  course: '',
  progress: 0
})

const filterStatus = ref('all')
const filterPriority = ref('all')

const filteredTasks = computed(() => {
  return tasks.value.filter(task => {
    const statusMatch = filterStatus.value === 'all' || task.status === filterStatus.value
    const priorityMatch = filterPriority.value === 'all' || task.priority === filterPriority.value
    return statusMatch && priorityMatch
  })
})

const addTask = () => {
  if (newTask.value.title) {
    tasks.value.push({
      id: tasks.value.length + 1,
      ...newTask.value
    })
    showAddTaskDialog.value = false
    // 重置表单
    newTask.value = {
      title: '',
      description: '',
      deadline: '',
      estimatedTime: 1,
      priority: 'medium',
      status: 'pending',
      course: '',
      progress: 0
    }
  }
}

const toggleTaskStatus = (task) => {
  task.status = task.status === 'completed' ? 'pending' : 'completed'
  if (task.status === 'completed') {
    task.progress = 100
  }
}

const getStatusText = (status) => {
  const statusMap = {
    pending: '未开始',
    inProgress: '进行中',
    completed: '已完成',
    overdue: '逾期'
  }
  return statusMap[status] || status
}

const getPriorityText = (priority) => {
  const priorityMap = {
    high: '高',
    medium: '中',
    low: '低'
  }
  return priorityMap[priority] || priority
}

// 编辑任务
const editTask = (task) => {
  // 这里可以实现编辑任务的逻辑
  alert('编辑任务功能开发中...')
}

// 删除任务
const deleteTask = (id) => {
  if (confirm('确定要删除这个任务吗？')) {
    tasks.value = tasks.value.filter(task => task.id !== id)
  }
}
</script>

<style scoped>
.tasks {
  padding: 20px 0;
}

h2 {
  margin-bottom: 20px;
  color: var(--primary-color);
}

.task-actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.task-filters {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.task-filters select {
  width: 200px;
}

.task-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.task-card {
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  overflow: hidden;
  background-color: white;
  border-left: 4px solid #e4e7ed;
}

.task-card.pending {
  border-left-color: var(--info-color);
}

.task-card.inProgress {
  border-left-color: var(--primary-color);
}

.task-card.completed {
  border-left-color: var(--success-color);
}

.task-card.overdue {
  border-left-color: var(--danger-color);
}

.task-header {
  padding: 16px;
  background-color: #f9f9f9;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.task-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
}

.task-priority {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.task-priority.high {
  background-color: rgba(245, 108, 108, 0.1);
  color: var(--danger-color);
}

.task-priority.medium {
  background-color: rgba(230, 162, 60, 0.1);
  color: var(--warning-color);
}

.task-priority.low {
  background-color: rgba(103, 194, 58, 0.1);
  color: var(--success-color);
}

.task-body {
  padding: 16px;
}

.task-info {
  margin-bottom: 12px;
}

.info-item {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--text-color-secondary);
}

.task-description {
  margin-bottom: 16px;
  font-size: 14px;
  line-height: 1.5;
}

.task-progress {
  margin-bottom: 16px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 4px;
}

.progress-fill {
  height: 100%;
  background-color: var(--primary-color);
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 12px;
  color: var(--text-color-secondary);
  float: right;
}

.task-footer {
  padding: 16px;
  background-color: #f9f9f9;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-tag {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
}

.status-tag.pending {
  background-color: rgba(144, 147, 153, 0.1);
  color: var(--info-color);
}

.status-tag.inProgress {
  background-color: rgba(64, 158, 255, 0.1);
  color: var(--primary-color);
}

.status-tag.completed {
  background-color: rgba(103, 194, 58, 0.1);
  color: var(--success-color);
}

.status-tag.overdue {
  background-color: rgba(245, 108, 108, 0.1);
  color: var(--danger-color);
}

.task-actions {
  display: flex;
  gap: 8px;
}

.btn-sm {
  padding: 4px 12px;
  font-size: 12px;
}

/* 对话框样式 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog-content {
  background-color: white;
  border-radius: var(--border-radius);
  padding: 24px;
  width: 400px;
  max-width: 90%;
}

.dialog-actions {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

@media (max-width: 768px) {
  .task-actions {
    flex-direction: column;
  }
  
  .task-filters {
    flex-direction: column;
  }
  
  .task-filters select {
    width: 100%;
  }
  
  .task-list {
    grid-template-columns: 1fr;
  }
}
</style>