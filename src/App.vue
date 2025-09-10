<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 定义数据类型
interface Stock {
  id?: number;
  symbol: string;
  name: string;
  price?: number;
  change_percent?: number;
  volume?: number;
  market_cap?: number;
  sector?: string;
  created_at?: string;
  updated_at?: string;
}

interface CreateStockRequest {
  symbol: string;
  name: string;
  price?: number;
  change_percent?: number;
  volume?: number;
  market_cap?: number;
  sector?: string;
}

// 响应式数据
const stocks = ref<Stock[]>([]);
const loading = ref(false);
const newStock = ref<CreateStockRequest>({
  symbol: '',
  name: '',
  price: 0,
  change_percent: 0,
  volume: 0,
  market_cap: 0,
  sector: ''
});

// 获取所有股票
const fetchStocks = async () => {
  try {
    loading.value = true;
    const result = await invoke<Stock[]>('get_all_stocks');
    stocks.value = result;
  } catch (error) {
    console.error('获取股票失败:', error);
  } finally {
    loading.value = false;
  }
};

// 创建新股票
const createStock = async () => {
  try {
    if (!newStock.value.symbol || !newStock.value.name) {
      alert('请填写股票代码和名称');
      return;
    }
    
    await invoke('create_stock', { stockData: newStock.value });
    
    // 清空表单
    newStock.value = {
      symbol: '',
      name: '',
      price: 0,
      change_percent: 0,
      volume: 0,
      market_cap: 0,
      sector: ''
    };
    
    // 重新获取股票列表
    await fetchStocks();
    alert('股票创建成功!');
  } catch (error) {
    console.error('创建股票失败:', error);
    alert('创建股票失败: ' + error);
  }
};

// 删除股票
const deleteStock = async (symbol: string) => {
  try {
    if (confirm(`确定要删除股票 ${symbol} 吗？`)) {
      await invoke('delete_stock', { symbol });
      await fetchStocks();
      alert('股票删除成功!');
    }
  } catch (error) {
    console.error('删除股票失败:', error);
    alert('删除股票失败: ' + error);
  }
};

// 获取股票历史
const getStockHistory = async (symbol: string) => {
  try {
    const history = await invoke('get_stock_history', { symbol, limit: 10 });
    console.log(`${symbol} 历史数据:`, history);
    alert(`${symbol} 历史数据已输出到控制台`);
  } catch (error) {
    console.error('获取历史数据失败:', error);
  }
};

// 数据库状态信息
const databaseStatus = ref<Record<string, string>>({});

// 获取数据库状态
const fetchDatabaseStatus = async () => {
  try {
    const result = await invoke<Record<string, string>>('get_database_info');
    databaseStatus.value = result;
    console.log('Database status:', result);
  } catch (error) {
    console.error('获取数据库状态失败:', error);
  }
};

// 触发数据库文件创建
const triggerDatabaseCreation = async () => {
  try {
    const result = await invoke<string>('create_database_files');
    console.log('Database creation result:', result);
    alert('数据库文件创建: ' + result);
    // 重新获取数据库状态
    await fetchDatabaseStatus();
  } catch (error) {
    console.error('数据库文件创建失败:', error);
    alert('数据库文件创建失败: ' + error);
  }
};

// 组件挂载时获取数据
onMounted(() => {
  fetchStocks();
  fetchDatabaseStatus();
});
</script>

<template>
  <main class="container">
    <div class="add-stock-form">
      <div class="form-grid">
        <div>
          <label>股票代码:</label>
          <input v-model="newStock.symbol" placeholder="例: AAPL" />
        </div>
        <div>
          <label>股票名称:</label>
          <input v-model="newStock.name" placeholder="例: Apple Inc." />
        </div>
        <div>
          <label>价格:</label>
          <input v-model.number="newStock.price" type="number" step="0.01" />
        </div>
        <div>
          <label>涨跌幅(%):</label>
          <input v-model.number="newStock.change_percent" type="number" step="0.01" />
        </div>
        <div>
          <label>成交量:</label>
          <input v-model.number="newStock.volume" type="number" />
        </div>
        <div>
          <label>市值:</label>
          <input v-model.number="newStock.market_cap" type="number" />
        </div>
        <div>
          <label>行业:</label>
          <input v-model="newStock.sector" placeholder="例: 科技" />
        </div>
      </div>
      <button @click="createStock" class="btn-primary">添加股票</button>
    </div>

    <!-- 股票列表 -->
    <div class="stocks-section">
      <div class="section-header">
        <h2>股票列表</h2>
        <div class="header-buttons">
          <button @click="fetchDatabaseStatus" class="btn-info">
            刷新数据库状态
          </button>
          <button @click="fetchStocks" :disabled="loading" class="btn-secondary">
            {{ loading ? '加载中...' : '刷新' }}
          </button>
        </div>
      </div>

      <!-- 数据库状态显示 -->
      <div class="database-status">
        <h3>数据库连接状态</h3>
        <div class="status-grid">
          <div v-for="(status, dbName) in databaseStatus" :key="dbName" class="status-item">
            <span class="db-name">{{ dbName }}:</span>
            <span class="db-status" :class="{ 'status-connected': status === '已连接' }">
              {{ status }}
            </span>
          </div>
        </div>
        <div class="database-actions">
          <button @click="triggerDatabaseCreation" class="btn-info">
            创建数据库文件
          </button>
        </div>
      </div>
      
      <div v-if="stocks.length === 0" class="empty-state">
        暂无股票数据
      </div>
      
      <div v-else class="stocks-grid">
        <div v-for="stock in stocks" :key="stock.symbol" class="stock-card">
          <div class="stock-header">
            <h3>{{ stock.symbol }}</h3>
            <span class="stock-name">{{ stock.name }}</span>
          </div>
          <div class="stock-details">
            <div v-if="stock.price" class="price">
              价格: ${{ stock.price.toFixed(2) }}
            </div>
            <div v-if="stock.change_percent" 
                 :class="['change', stock.change_percent >= 0 ? 'positive' : 'negative']">
              {{ stock.change_percent >= 0 ? '+' : '' }}{{ stock.change_percent.toFixed(2) }}%
            </div>
            <div v-if="stock.volume" class="volume">
              成交量: {{ stock.volume.toLocaleString() }}
            </div>
            <div v-if="stock.sector" class="sector">
              行业: {{ stock.sector }}
            </div>
          </div>
          <div class="stock-actions">
            <button @click="getStockHistory(stock.symbol)" class="btn-info">
              查看历史
            </button>
            <button @click="deleteStock(stock.symbol)" class="btn-danger">
              删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.add-stock-form {
  background: #f5f5f5;
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 30px;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  margin-bottom: 20px;
}

.form-grid div {
  display: flex;
  flex-direction: column;
}

.form-grid label {
  margin-bottom: 5px;
  font-weight: bold;
}

.form-grid input {
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.stocks-section {
  margin-top: 30px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-buttons {
  display: flex;
  gap: 10px;
}

.stocks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.stock-card {
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.stock-header h3 {
  margin: 0 0 5px 0;
  color: #333;
}

.stock-name {
  color: #666;
  font-size: 14px;
}

.stock-details {
  margin: 15px 0;
}

.stock-details div {
  margin: 5px 0;
}

.price {
  font-size: 18px;
  font-weight: bold;
  color: #333;
}

.change.positive {
  color: #4caf50;
}

.change.negative {
  color: #f44336;
}

.stock-actions {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

.btn-primary {
  background: #2196f3;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-secondary {
  background: #9e9e9e;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-info {
  background: #17a2b8;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.btn-danger {
  background: #dc3545;
  color: white;
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.btn-primary:hover, .btn-secondary:hover, .btn-info:hover, .btn-danger:hover {
  opacity: 0.8;
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.empty-state {
  text-align: center;
  color: #666;
  padding: 40px;
  background: #f9f9f9;
  border-radius: 8px;
}

/* 数据库状态样式 */
.database-status {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 16px;
  margin: 16px 0;
}

.database-status h3 {
  margin: 0 0 12px 0;
  color: #495057;
  font-size: 16px;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 8px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: white;
  border: 1px solid #e9ecef;
  border-radius: 4px;
}

.db-name {
  font-weight: 600;
  color: #343a40;
}

.db-status {
  font-size: 14px;
  padding: 2px 8px;
  border-radius: 12px;
  background: #ffc107;
  color: #212529;
}

.status-connected {
  background: #28a745 !important;
  color: white !important;
}

.database-actions {
  margin-top: 12px;
  display: flex;
  gap: 8px;
}

.database-actions .btn-info {
  font-size: 14px;
  padding: 8px 16px;
}
</style>