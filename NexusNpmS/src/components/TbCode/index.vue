<template>
  <div class="necode">
    <div 
      class="codebox"
      :style="{
        'background': codeback,
        'width': width + 'px',
        'height': height + 'px'
      }"
      @click="getCode(length)"
    >
      {{ codevalue }}
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

// 接收父组件传递的 props
defineProps({
  value: {
    type: String,
    required: false,
  },
  length: {
    type: Number,
    default: 4,
    required: false,
  },
  back: {
    type: String,
    required: false,
  },
  width: {
    type: Number,
    default: 120,  // 默认宽度为120px
  },
  height: {
    type: Number,
    default: 40,   // 默认高度为40px
  }
});


const codelength = ref(4);

// 响应式变量
const codevalue = ref(''); // 验证码值
const codeback = ref('');  // 验证码背景色

// onMounted 是 Vue 3 的生命周期钩子，类似于 Vue 2 的 created
onMounted(() => {
  codelength.value=length?length:codelength.value;
  getCode(codelength.value); // 获取验证码
});

// 新增试用码-前端随机生成方法
const getCode = (row) => {
  // 随机背景颜色
  const r = Math.floor(Math.random() * 256);
  const g = Math.floor(Math.random() * 256);
  const b = Math.floor(Math.random() * 256);
  const rgb = `rgb(${r},${g},${b})`;
  codeback.value = rgb;

  const arrall = [
    'A', 'B', 'C', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
  ];
  let str = '';
  for (let i = 0; i < row; i++) {
    str += arrall[Math.floor(Math.random() * arrall.length)];
  }
  codevalue.value = str;
};
</script>

<style scoped>
.codebox {
  text-align: center;
  font-weight: 800;
  line-height: 40px;
  display: inline-block;
  cursor: pointer;
  font-size: 24px;
  color: #fff;
  border-radius: 4px;
}
</style>
