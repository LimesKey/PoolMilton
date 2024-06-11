<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';

    let time: string;
    let services = [
        { title: "Water Analysis", description: "Comprehensive chemical analysis of your pool water to ensure safety and clarity.", image: "/src/assets/taylor-test-kit.png" },
        { title: "Filter Cleaning", description: "Cleans skimmer basket and pump filter every visit, will wash cartridge filters if required.", image: "/src/assets/cartridge-filter.png" },
        { title: "Monthly Maintenance", description: "We're able to do light routine maintence tasks like washing the stones and sides of the pool.\n \t We can also wash out your salt water chlorinator cell to ensure it's functioning at it's maximum.", image: "/src/assets/pool-pressure-washing.jpg" },
    ];
    let pricing = [
        { plan: "Twice a Week", price: "$120/month", description: "Best for frequent pool-goers and for people who have either a really large or really small pool. .", responsibilities: "Includes water testing, purchasing and adding chemicals and minor cleaning tasks included." },
        { plan: "Once a Week", price: "$70/month", description: "Regular weekly testing to maintain the balance and cleanliness of your pool.", responsibilities: "Includes weekly chemical testing and balancing. One-time contaminant check per week." },
        { plan: "Once a Month", price: "$30/month", description: "Basic monthly testing for low-maintenance pools.", responsibilities: "Monthly chemical testing and balancing. No cleaning tasks included." }
    ];

    onMount(async () => {
        try {
            time = await invoke('display_time');
            console.log(time);
        } catch (error) {
            console.error('Error occurred:', error);
        }
    });
</script>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&family=Lobster&display=swap');
    
    :global(body) {
        margin: 0;
        font-family: 'Roboto', sans-serif;
        background-color: #121212;
        color: #f1f1f1;
    }
    header {
        background: #1e88e5;
        padding: 1rem 0;
        text-align: center;
        color: white;
    }
    nav a {
        margin: 0 15px;
        color: white;
        text-decoration: none;
        font-weight: bold;
    }
    .title {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 80vh;
        background: linear-gradient(135deg, rgba(30, 136, 229, 0.8) 0%, rgba(66, 165, 245, 0.8) 100%), url('/src/assets/pool-side.jpg') no-repeat center center;
        background-size: cover;
        color: white;
        text-align: center;
    }
    .title h1 {
        font-family: 'Lobster', cursive;
        font-size: 3.5rem;
        margin-bottom: 0.5rem;
    }
    .title p {
        font-size: 1.5rem;
        margin-bottom: 2rem;
    }
    .button {
        padding: 1rem 2rem;
        background: white;
        color: #1e88e5;
        border: none;
        border-radius: 25px;
        cursor: pointer;
        font-size: 1.2rem;
        transition: background 0.3s;
    }
    .button:hover {
        background: #f1f1f1;
    }
    .section {
        padding: 4rem 2rem;
    }
    .dark-section {
        background: #212121;
    }
    .section h2 {
        text-align: center;
        margin-bottom: 3rem;
        font-family: 'Lobster', cursive;
        font-size: 2.5rem;
    }
    .item {
        background: #333;
        padding: 2rem;
        border-radius: 10px;
        margin-bottom: 2rem;
        transition: transform 0.3s;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    .item:hover {
        transform: translateY(-10px);
    }
    .item h3 {
        margin-bottom: 1rem;
        font-family: 'Lobster', cursive;
        font-size: 1.8rem;
    }
    .item p {
        font-family: 'Roboto', sans-serif;
        font-size: 1.2rem;
    }
    .pricing-container {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .service-item img {
        max-width: 40%;
        max-width: 30%;
        height: auto;
        border-radius: 10px;
        margin-left: 2rem;
        margin-bottom: 1rem;

    }
    .service-item {
        display: flex;
        flex-direction: row-reverse;
        align-items: center;
    }
    .service-item-content {
        flex: 1;
    }
</style>

<header>
    <nav>
        <a href="#home">Home</a>
        <a href="#services">Services</a>
        <a href="#pricing">Pricing</a>
        <a href="#contact">Contact</a>
    </nav>

    <link rel="icon" href="/src/assets/splash.ico" type="image/x-con">
</header>

<section id="home" class="title">
    <h1>Milton Pool Water Testing</h1>
    <p>Ensuring your pool water is safe, clean, and crystal clear.</p>
    <button class="button">Get Started</button>
</section>

<section id="services" class="section dark-section">
    <h2>Our Services</h2>
    {#each services as service}
        <div class="item service-item">
            <img src={service.image} alt={service.title} />
            <div class="service-item-content">
                <h3>{service.title}</h3>
                <p>{service.description}</p>
            </div>
        </div>
    {/each}
</section>

<section id="pricing" class="section dark-section">
    <h2>Pricing Options</h2>
    <div class="pricing-container">
        {#each pricing as plan}
            <div class="item">
                <h3>{plan.plan}</h3>
                <p>{plan.price}</p>
                <p>{plan.description}</p>
                <p><strong>Responsibilities:</strong> {plan.responsibilities}</p>
            </div>
        {/each}
    </div>
</section>

<section id="contact" class="section">
    <h2>Contact Us</h2>
    <p>Email: info@ryandl.com</p>
    <p>Phone: (647)-803-8955</p>
</section>