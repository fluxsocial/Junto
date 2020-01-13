<template>
    <section class="profile">
        <div class="profile--card-container">
            <div class="profile--card">
                <div class="profile--card-content-left">

                </div>
                <div class="profile--card-content-right">
                    <div class="profile--info">
                        <p class="profile--info-name">{{ profile.entry.first_name }} {{ profile.entry.last_name }}</p>
                        <p class="profile--info-username">@{{ username.entry.username }}</p>
                        <p class="profile--info-bio">{{ profile.entry.bio }}</p>
                    </div>
                    <div class="profile--stats">
                        <div class="profile--stats-item stats-expressions">
                            <h6>Expressions</h6>
                            <span>44</span>
                        </div>
                        <div class="profile--stats-item stats-groups">
                            <h6>Groups</h6>
                            <span>2</span>
                        </div>
                        <div class="profile--stats-item stats-packs">
                            <h6>Packs</h6>
                            <span>3</span>
                        </div>
                    </div>
                    <div class="profile--links">
                        <div class="profile--links-btn-container">
                            <router-link class="btn profile--links-public-btn" :to="address + `/den/public`">
                                Public Den
                            </router-link>
                        </div>
                        <div class="profile--links-btn-container">
                            <router-link class="btn profile--links-private-btn" :to="address + `/den/private`">
                                Private Den
                            </router-link>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>

<script>
import profileHttpMethods from './ProfileHttp';

export default {
    name: "Profile", 
    props: {
        username: Object,
        profile: Object,
        address: String
    }
}
</script>