<template>
  <div class="courses">
    <div class="container">
      <h2>课程管理</h2>
      
      <!-- 课程操作栏 -->
      <div class="course-actions">
        <button class="btn btn-primary" @click="showAddCourseDialog = true">添加课程</button>
        <button class="btn btn-success" @click="importCourse">导入课表</button>
        <button class="btn btn-warning" @click="shareCourse">分享课表</button>
      </div>
      
      <!-- 课程列表 -->
      <div class="card">
        <h3>我的课程</h3>
        <div class="course-list">
          <div v-for="course in courses" :key="course.id" class="course-card">
            <div class="course-header" :style="{ backgroundColor: course.color }">{{ course.name }}</div>
            <div class="course-body">
              <div class="course-info">
                <span class="info-item">📅 {{ course.weekdays }}</span>
                <span class="info-item">⏰ {{ course.time }}</span>
                <span class="info-item">🏫 {{ course.location }}</span>
                <span class="info-item">👨‍🏫 {{ course.teacher }}</span>
              </div>
              <div class="course-actions">
                <button class="btn btn-sm btn-primary" @click="editCourse(course)">编辑</button>
                <button class="btn btn-sm btn-danger" @click="deleteCourse(course.id)">删除</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 课程表视图 -->
      <div class="card">
        <h3>课程表</h3>
        <div class="timetable">
          <div class="timetable-header">
            <div class="time-slot"></div>
            <div class="day-slot" v-for="day in weekdays" :key="day">{{ day }}</div>
          </div>
          <div class="timetable-body">
            <div v-for="slot in timeSlots" :key="slot.id" class="time-row">
              <div class="time-slot">{{ slot.time }}</div>
              <div v-for="day in weekdays" :key="`${day}-${slot.id}`" class="course-slot">
                <div v-for="course in getCourseByTime(day, slot.id)" :key="course.id" 
                     class="course-item" :style="{ backgroundColor: course.color }">
                  {{ course.name }}
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 添加课程对话框 -->
      <div v-if="showAddCourseDialog" class="dialog-overlay" @click="showAddCourseDialog = false">
        <div class="dialog-content" @click.stop>
          <h3>添加课程</h3>
          <div class="form-group">
            <label class="form-label">课程名称</label>
            <input type="text" class="form-control" v-model="newCourse.name">
          </div>
          <div class="form-group">
            <label class="form-label">上课时间</label>
            <input type="text" class="form-control" v-model="newCourse.time" placeholder="如：08:00-09:40">
          </div>
          <div class="form-group">
            <label class="form-label">上课地点</label>
            <input type="text" class="form-control" v-model="newCourse.location">
          </div>
          <div class="form-group">
            <label class="form-label">任课老师</label>
            <input type="text" class="form-control" v-model="newCourse.teacher">
          </div>
          <div class="form-group">
            <label class="form-label">上课星期</label>
            <input type="text" class="form-control" v-model="newCourse.weekdays" placeholder="如：周一、周三">
          </div>
          <div class="form-group">
            <label class="form-label">课程颜色</label>
            <input type="color" class="form-control" v-model="newCourse.color">
          </div>
          <div class="dialog-actions">
            <button class="btn btn-primary" @click="addCourse">确定</button>
            <button class="btn" @click="showAddCourseDialog = false">取消</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// 模拟数据
const courses = ref([
  { id: 1, name: '高等数学', time: '08:00-09:40', location: '教学楼A101', teacher: '张老师', weekdays: '周一、周三', color: '#409EFF' },
  { id: 2, name: '大学英语', time: '14:00-15:40', location: '教学楼B202', teacher: '李老师', weekdays: '周二、周四', color: '#67C23A' },
  { id: 3, name: '计算机基础', time: '10:00-11:40', location: '实验楼C303', teacher: '王老师', weekdays: '周一、周五', color: '#E6A23C' }
])

const showAddCourseDialog = ref(false)
const newCourse = ref({
  name: '',
  time: '',
  location: '',
  teacher: '',
  weekdays: '',
  color: '#409EFF'
})

const weekdays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日']
const timeSlots = [
  { id: 1, time: '08:00-09:40' },
  { id: 2, time: '10:00-11:40' },
  { id: 3, time: '14:00-15:40' },
  { id: 4, time: '16:00-17:40' },
  { id: 5, time: '19:00-20:40' }
]

const addCourse = () => {
  if (newCourse.value.name) {
    courses.value.push({
      id: courses.value.length + 1,
      ...newCourse.value
    })
    showAddCourseDialog.value = false
    // 重置表单
    newCourse.value = {
      name: '',
      time: '',
      location: '',
      teacher: '',
      weekdays: '',
      color: '#409EFF'
    }
  }
}

const getCourseByTime = (day, slotId) => {
  return courses.value.filter(course => 
    course.weekdays.includes(day) && 
    course.time === timeSlots.find(slot => slot.id === slotId).time
  )
}

// 导入课表
const importCourse = () => {
  // 这里可以实现导入课表的逻辑，例如从文件导入或对接教务系统
  alert('导入课表功能开发中...')
}

// 分享课表
const shareCourse = () => {
  // 这里可以实现分享课表的逻辑，例如生成分享链接或图片
  alert('分享课表功能开发中...')
}

// 编辑课程
const editCourse = (course) => {
  // 这里可以实现编辑课程的逻辑
  alert('编辑课程功能开发中...')
}

// 删除课程
const deleteCourse = (id) => {
  if (confirm('确定要删除这门课程吗？')) {
    courses.value = courses.value.filter(course => course.id !== id)
  }
}
</script>

<style scoped>
.courses {
  padding: 20px 0;
}

h2 {
  margin-bottom: 20px;
  color: var(--primary-color);
}

.course-actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.course-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.course-card {
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  overflow: hidden;
}

.course-header {
  padding: 16px;
  color: white;
  font-weight: 500;
  font-size: 16px;
}

.course-body {
  padding: 16px;
  background-color: white;
}

.course-info {
  margin-bottom: 16px;
}

.info-item {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--text-color-secondary);
}

.course-actions {
  display: flex;
  gap: 10px;
}

.btn-sm {
  padding: 4px 12px;
  font-size: 12px;
}

/* 课程表样式 */
.timetable {
  margin-top: 20px;
  border-collapse: collapse;
  width: 100%;
}

.timetable-header {
  display: flex;
  background-color: #f5f7fa;
  border-bottom: 1px solid var(--border-color);
}

.time-slot {
  width: 100px;
  padding: 12px;
  text-align: center;
  border-right: 1px solid var(--border-color);
  font-weight: 500;
}

.day-slot {
  flex: 1;
  padding: 12px;
  text-align: center;
  border-right: 1px solid var(--border-color);
  font-weight: 500;
}

.day-slot:last-child {
  border-right: none;
}

.timetable-body {
  display: flex;
  flex-direction: column;
}

.time-row {
  display: flex;
  border-bottom: 1px solid var(--border-color);
}

.course-slot {
  flex: 1;
  padding: 4px;
  border-right: 1px solid var(--border-color);
  min-height: 80px;
}

.course-slot:last-child {
  border-right: none;
}

.course-item {
  padding: 8px;
  border-radius: 4px;
  color: white;
  font-size: 12px;
  margin-bottom: 4px;
  text-align: center;
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
  .course-actions {
    flex-direction: column;
  }
  
  .course-list {
    grid-template-columns: 1fr;
  }
  
  .timetable {
    overflow-x: auto;
  }
  
  .time-slot {
    width: 80px;
  }
}
</style>