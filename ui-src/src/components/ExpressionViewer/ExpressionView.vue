<template>
  <section>
    <shortForm v-if="isShortForm" short-form-data="shortFormData"></shortForm>
    <longForm v-if="isLongForm" long-form-data="longFormData"></longForm>
  </section>
</template>

<script>
import ShortForm from "./Shortform/Shortform";
import LongForm from "./LongForm/Storypreview";

export default {
  name: "ExpressionView",
  components: {
    shortForm: ShortForm,
    longForm: LongForm
  },

  props: {},
  data() {
    return {
      isLongForm: false,
      isShortForm: false,
      longFormData: {
        title: String,
        body: String,
        users_name: String,
        username: String
      },
      shortFormData: {
        users_name: String,
        username: String,
        text: String
      }
    };
  },

  methods: {
    renderExpression(data) {
      if (data.type == "ShortForm") {
        this.isShortForm = true;
      } else if (data.type == "LongForm") {
        this.isLongForm = true;
      }
    }
  }
};
</script>
