<template>
  <section class="shortform-view">

  </section>
</template>

<script>
import expressionViewerHttpMethods from "./../ExpressionViewerHttp";

export default {
  name: "Shortform",
  data() {
    return {
      perspective: String,
      attributes: Array,
      queryOptions: String,
      targetType: String,
      queryType: String,
      dos: Number,
      seed: String
    };
  },
  methods: {
    getExpression() {
      expressionViewerHttpMethods.getExpression(
        this.perspective,
        this.attributes,
        this.queryOptions,
        this.targetType,
        this.queryType,
        this.dos,
        this.seed
      );
    }
  }
};
</script>
