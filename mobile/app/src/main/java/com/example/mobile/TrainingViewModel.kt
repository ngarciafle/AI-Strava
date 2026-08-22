package com.example.mobile

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class TrainingViewModel: ViewModel() {
    private val motorRust = Training()

    private val _stats = MutableStateFlow<StatsTraining>(
        StatsTraining(
            distance = 0.0,
            elevationGain = 0.0,
            elevationLoss = 0.0,
            rithm = 0.0,
            time = 0.0,
            rithms = listOf()
        )
    )
    val stats = _stats.asStateFlow()

    fun registerPoint(lat: Double, lon: Double, alt: Double, time: Double) {
        val newStats = motorRust.registerNewPoint(lat, lon, alt, time)
        _stats.value = newStats
    }

    fun endTraining() {
        motorRust.endTraining()
    }
}