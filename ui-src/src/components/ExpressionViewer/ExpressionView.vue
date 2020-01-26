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
      if (data.expression.entry.expression_type == "ShortForm") {
        this.isShortForm = true;
        this.shortFormData = {
          text: data.expression.entry.expression.text,
          users_name: data.author_profile.entry.first_name + " " + data.author_profile.entry.last_name,
          username: data.author_username.entry.username
        };
      } else if (data.expression.entry.expression_type == "LongForm") {
        this.isLongForm = true;
        this.shortFormData = {
          title: data.expression.entry.expression.title,
          body: data.expression.entry.expression.body,
          users_name: data.author_profile.entry.first_name + " " + data.author_profile.entry.last_name,
          username: data.author_username.entry.username
        };
      }
    }
  }
};
</script>
