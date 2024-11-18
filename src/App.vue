<template>
  <div class="file-counter">
    <h1>File Counter</h1>

    <!-- Bouton pour déclencher le comptage -->
    <button @click="fetchFileCount" :disabled="loading">
      Start Counting Files
    </button>

    <!-- Spinner et progression pendant le chargement -->
    <div v-if="loading" class="spinner">
      <p>Counting files... {{ progress }} files counted.</p>
      <p v-if="errorCount > 0">Errors encountered: {{ errorCount }}</p>
    </div>

    <!-- Affichage du résultat final -->
    <div v-if="!loading && fileCount !== null">
      <p>Total files: {{ fileCount }}</p>
    </div>

    <!-- Dernière erreur -->
    <div v-if="error">
      <p class="error">Last error: {{ error }}</p>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";

type FileCountEvent =
  | {
      event: "progress";
      data: {
        count: number;
        currentPath: string;
      };
    }
  | {
      event: "finished";
      data: {
        totalCount: number;
      };
    }
  | {
      event: "error";
      data: {
        message: string;
      };
    };

export default defineComponent({
  name: "FileCounter",
  data() {
    return {
      fileCount: null as number | null,
      currentCount: 0,
      errorCount: 0,
      error: null as string | null,
      loading: false,
    };
  },
  computed: {
    progress(): number {
      return this.currentCount;
    },
  },
  methods: {
    async fetchFileCount() {
      this.loading = true;
      this.error = null;
      this.fileCount = null;
      this.currentCount = 0;
      this.errorCount = 0;

      try {
        const onEvent = new Channel<FileCountEvent>();

        onEvent.onmessage = (message) => {
          switch (message.event) {
            case "progress":
              this.currentCount = message.data.count;
              break;
            case "finished":
              this.fileCount = message.data.totalCount;
              this.loading = false;
              break;
            case "error":
              this.error = message.data.message; // Met à jour avec la dernière erreur
              this.errorCount += 1; // Incrémente le compteur d'erreurs
              break;
          }
        };

        await invoke("get_file_count", { onEvent });
      } catch (err) {
        this.error = (err as Error).message || "An unknown error occurred.";
        this.loading = false;
      }
    },
  },
});
</script>

<style scoped>
.file-counter {
  text-align: center;
  margin: 20px;
}

button {
  padding: 10px 20px;
  margin-top: 20px;
  font-size: 16px;
  cursor: pointer;
}

.spinner {
  margin: 20px;
  font-size: 18px;
  font-weight: bold;
  color: #007bff;
}

.error {
  color: red;
  margin-top: 20px;
}

p {
  margin: 10px;
  font-size: 16px;
}
</style>
